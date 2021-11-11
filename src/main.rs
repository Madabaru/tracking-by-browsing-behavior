pub mod cli;
pub mod maths;
pub mod metrics;
pub mod parse;
pub mod click_trace;
pub mod utils;

use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufWriter;
use std::str::FromStr;

use rayon::prelude::*;

use rand::{rngs::StdRng, SeedableRng};
use rand::{seq::IteratorRandom, Rng};

use ordered_float::OrderedFloat;
use parse::DataFields;
use click_trace::{ClickTrace, VectClickTrace};
use metrics::DistanceMetric;

fn main() {
    // Load config
    let config = cli::get_cli_config().unwrap();

    // Set random seed for reproducability
    let mut rng = StdRng::seed_from_u64(config.seed);

    let client_to_hist_map: HashMap<u32, Vec<ClickTrace>> = parse::parse_to_hist(&config).unwrap();

    let client_to_target_idx_map: HashMap<u32, usize> =
        gen_test_data(&client_to_hist_map, &mut rng, config.client_sample_size);

    if !config.typical {
        let client_to_sample_idx_map: HashMap<u32, Vec<usize>> = get_train_data(
            &client_to_hist_map,
            &mut rng,
            config.click_trace_sample_size,
        );

        eval(
            &config,
            &client_to_hist_map,
            &client_to_target_idx_map,
            &client_to_sample_idx_map,
        );
    } else {
        let client_to_sample_idx_map: HashMap<u32, Vec<usize>> =
            get_train_data(&client_to_hist_map, &mut rng, 0);

        eval(
            &config,
            &client_to_hist_map,
            &client_to_target_idx_map,
            &client_to_sample_idx_map,
        );
    }
}

// Sample a subset of clients and a target click trace that the evaluation is based upon
fn gen_test_data<R: Rng>(
    client_to_hist_map: &HashMap<u32, Vec<ClickTrace>>,
    rng: &mut R,
    client_sample_size: usize,
) -> HashMap<u32, usize> {
    let client_list: Vec<u32> = client_to_hist_map.keys().cloned().collect();
    // Randomly sample clients from the client list
    let sampled_clients_list = client_list.iter().choose_multiple(rng, client_sample_size);
    let mut client_to_target_idx_map = HashMap::<u32, usize>::new();
    for client in sampled_clients_list.into_iter() {
        let click_traces_list = &client_to_hist_map.get(client).unwrap();
        let click_trace_len = click_traces_list.len();
        // Split click history in 50%/50%
        let split_idx = click_trace_len / 2;
        let rand_target_idx: usize = rng.gen_range(split_idx..click_trace_len);
        client_to_target_idx_map.insert(*client, rand_target_idx);
    }
    return client_to_target_idx_map;
}

// Sample click traces for each client and store sample indices in map
fn get_train_data<R: Rng>(
    client_to_hist_map: &HashMap<u32, Vec<ClickTrace>>,
    rng: &mut R,
    click_trace_sample_size: usize,
) -> HashMap<u32, Vec<usize>> {
    let mut client_to_sample_idx_map: HashMap<u32, Vec<usize>> = HashMap::new();
    for (client, click_traces_list) in client_to_hist_map.into_iter() {
        let client = client.clone();
        let click_trace_len = click_traces_list.len();
        let split_idx = click_trace_len / 2;
        let indices: Vec<usize> = (0..split_idx).collect();
        if click_trace_sample_size > 0 {
            let sampled_idx = indices
                .into_iter()
                .choose_multiple(rng, click_trace_sample_size);
            client_to_sample_idx_map.insert(client, sampled_idx);
        } else {
            client_to_sample_idx_map.insert(client, indices);
        }
    }
    client_to_sample_idx_map
}

fn eval(
    config: &cli::Config,
    client_to_hist_map: &HashMap<u32, Vec<ClickTrace>>,
    client_to_target_idx_map: &HashMap<u32, usize>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) {
    let result_list: Vec<(u32, u32, bool, bool)> = client_to_target_idx_map
        .par_iter()
        .map(|(client, target_idx)| {
            eval_step(
                config,
                client,
                target_idx,
                &client_to_hist_map,
                client_to_sample_idx_map,
            )
        })
        .collect();

    let mut correct_pred = 0;
    let mut top_10_count = 0;
    let mut top_10_percent_count = 0;
    for (pred, target, in_top_10, in_top_10_percent) in result_list.iter() {
        if pred == target {
            correct_pred += 1
        }
        if *in_top_10 {
            top_10_count += 1;
        }
        if *in_top_10_percent {
            top_10_percent_count += 1;
        }
    }

    let accuracy: f64 = correct_pred as f64 / result_list.len() as f64;
    println!("Rank 1: {:?}", accuracy);
    let top_10: f64 = top_10_count as f64 / result_list.len() as f64;
    println!("Top 10: {:?}", top_10);
    let top_10_percent: f64 = top_10_percent_count as f64 / result_list.len() as f64;
    println!("Top 10 Percent: {:?}", top_10_percent);

    let file = File::create("tmp/output").unwrap();
    let mut writer = BufWriter::new(&file);
    for i in result_list {
        write!(writer, "{},{} \n", i.0, i.1).expect("Unable to write to output file.");
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tmp/evaluation")
        .unwrap();
    write!(
        file,
        "-----------------------------------------------\nExperiment: {:?}\nTop 10: {}\nTop 10 Percent: {}\n",
        config, top_10, top_10_percent
    )
    .expect("Unable to write to evaluation file.")
}

fn eval_step(
    config: &cli::Config,
    client_target: &u32,
    target_idx: &usize,
    client_to_hist_map: &HashMap<u32, Vec<ClickTrace>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) -> (u32, u32, bool, bool) {
    let metric = metrics::DistanceMetric::from_str(&config.metric).unwrap();
    let target_hist = client_to_hist_map
        .get(client_target)
        .unwrap()
        .get(*target_idx)
        .unwrap();

    let mut tuples: Vec<(OrderedFloat<f64>, u32)> = Vec::with_capacity(client_to_hist_map.len());

    for (client, click_traces) in client_to_hist_map.into_iter() {
        let samples_idx = client_to_sample_idx_map.get(client).unwrap();
        let sampled_hists: Vec<ClickTrace> = samples_idx
            .into_iter()
            .map(|idx| click_traces.get(*idx).unwrap().clone())
            .collect();

        let (website_set, code_set, location_set, category_set) =
            utils::get_unique_sets(target_hist, &sampled_hists);

        let vectorized_target = click_trace::vectorize_click_trace(
            target_hist,
            &website_set,
            &code_set,
            &location_set,
            &category_set,
        );

        if config.typical {
            let vect_typ_click_trace = click_trace::gen_typ_vectorized_click_trace(
                &sampled_hists,
                &website_set,
                &code_set,
                &location_set,
                &category_set,
            );
            let dist = compute_dist(
                &config.fields,
                &metric,
                &vectorized_target,
                &vect_typ_click_trace,
            );
            tuples.push((OrderedFloat(dist), client.clone()));
        } else {
            for sample_hist in sampled_hists.into_iter() {
                let vectorized_ref = click_trace::vectorize_click_trace(
                    &sample_hist,
                    &website_set,
                    &code_set,
                    &location_set,
                    &category_set,
                );
                let dist =
                    compute_dist(&config.fields, &metric, &vectorized_target, &vectorized_ref);
                tuples.push((OrderedFloat(dist), client.clone()));
            }
        }
    }
    tuples.sort_unstable_by_key(|k| k.0);
    let cutoff: usize = (0.1 * client_to_hist_map.len() as f64) as usize;
    let is_top_10_percent = utils::is_target_in_top_k(client_target, &tuples[..cutoff]);
    let is_top_10: bool = utils::is_target_in_top_k(client_target, &tuples[..1]);
    (
        client_target.clone(),
        tuples[0].1,
        is_top_10,
        is_top_10_percent,
    )
}

// Calculate the distance between the target and the reference click trace
fn compute_dist(
    fields: &Vec<DataFields>,
    metric: &DistanceMetric,
    target_click_trace: &VectClickTrace,
    ref_click_trace: &VectClickTrace,
) -> f64 {
    // Vector to store distance scores for each data field to be considered
    let mut total_dist = Vec::<f64>::with_capacity(fields.len());

    // Iterate over all data fields that are considered
    for field in fields.into_iter() {
        let (target_vector, ref_vector) = match field {
            DataFields::Website => (
                target_click_trace.website.clone(),
                ref_click_trace.website.clone(),
            ),
            DataFields::Code => (
                target_click_trace.code.clone(),
                ref_click_trace.code.clone(),
            ),
            DataFields::Location => (
                target_click_trace.location.clone(),
                ref_click_trace.location.clone(),
            ),
            DataFields::Category => (
                target_click_trace.category.clone(),
                ref_click_trace.category.clone(),
            ),
            DataFields::Day => (target_click_trace.day.clone(), ref_click_trace.day.clone()),
            DataFields::Hour => (
                target_click_trace.hour.clone(),
                ref_click_trace.hour.clone(),
            ),
        };

        let dist = match metric {
            DistanceMetric::Euclidean => metrics::euclidean_dist(target_vector, ref_vector),
            DistanceMetric::Manhatten => metrics::manhatten_dist(target_vector, ref_vector),
            DistanceMetric::Cosine => metrics::consine_dist(target_vector, ref_vector),
            DistanceMetric::Jaccard => metrics::jaccard_dist(target_vector, ref_vector),
            DistanceMetric::Bhattacharyya => metrics::bhattacharyya_dist(target_vector, ref_vector),
            DistanceMetric::KullbrackLeibler => metrics::kl_dist(target_vector, ref_vector),
            DistanceMetric::TotalVariation => metrics::total_var_dist(target_vector, ref_vector),
            DistanceMetric::JeffriesMatusita => metrics::jeffries_dist(target_vector, ref_vector),
            DistanceMetric::ChiSquared => metrics::chi_squared_dist(target_vector, ref_vector),
        };
        total_dist.push(dist);
    }

    // Compute the final score by averaging the indivdual scores
    let avg_dist = total_dist.iter().sum::<f64>() / total_dist.len() as f64;
    avg_dist
}
