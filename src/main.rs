pub mod maths;
pub mod metrics;
pub mod parse;
pub mod structs;
pub mod utils;

use std::collections::HashMap;
use std::str::FromStr;

use rayon::prelude::*;

use rand::{rngs::StdRng, SeedableRng};
use rand::{seq::IteratorRandom, Rng};

use parse::DataFields;
use structs::{ClickTrace, ClickTraceVectorized};

use metrics::DistanceMetrics;

use ini::{Ini, Properties};

fn main() {
    // Load config file
    let ini = Ini::load_from_file("conf.ini").unwrap();
    let conf = ini.section(Some("Config")).unwrap();

    // Set random seed for reproducability
    let seed = conf.get("seed").unwrap().parse::<u64>().unwrap();
    let mut rng = StdRng::seed_from_u64(seed);

    let client_to_histogram_map: HashMap<u32, Vec<ClickTrace>> =
        parse::parse_to_histogram(conf).unwrap();

    let client_sample_size = conf
        .get("client_sample_size")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let client_to_target_idx_map: HashMap<u32, usize> =
        gen_test_data(&client_to_histogram_map, &mut rng, client_sample_size);

    let is_typical_session = conf
        .get("is_typical_session")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    if !is_typical_session {

        let click_trace_sample_size_per_client = conf
            .get("click_trace_sample_size_per_client")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let client_to_sample_idx_map: HashMap<u32, Vec<usize>> = get_train_data(
            &client_to_histogram_map,
            &mut rng,
            click_trace_sample_size_per_client,
        );

        eval(
            conf,
            &client_to_histogram_map,
            &client_to_target_idx_map,
            Some(&client_to_sample_idx_map),
        );
    } else {
        eval(
            conf,
            &client_to_histogram_map,
            &client_to_target_idx_map,
            None,
        );
    }
}

// Sample a subset of clients and a target click trace that the evaluation is based upon
fn gen_test_data<R: Rng>(
    client_to_histogram_map: &HashMap<u32, Vec<ClickTrace>>,
    rng: &mut R,
    client_sample_size: usize,
) -> HashMap<u32, usize> {
    let client_list: Vec<u32> = client_to_histogram_map.keys().cloned().collect();
    // Randomly sample clients from the client list
    let sampled_clients_list = client_list.iter().choose_multiple(rng, client_sample_size);
    let mut client_to_target_idx_map = HashMap::<u32, usize>::new();
    for client in sampled_clients_list.into_iter() {
        let click_traces_list = &client_to_histogram_map.get(client).unwrap();
        let click_trace_len = click_traces_list.len();
        let split_idx = click_trace_len / 2;
        let rand_target_idx: usize = rng.gen_range(split_idx..click_trace_len);
        client_to_target_idx_map.insert(*client, rand_target_idx);
    }
    return client_to_target_idx_map;
}

// Sample click traces for each client and store sample indices in map
fn get_train_data<R: Rng>(
    client_to_histogram_map: &HashMap<u32, Vec<ClickTrace>>,
    rng: &mut R,
    click_trace_sample_size_per_client: usize,
) -> HashMap<u32, Vec<usize>> {
    let mut client_to_sample_idx_map: HashMap<u32, Vec<usize>> = HashMap::new();
    for (client, click_traces_list) in client_to_histogram_map.into_iter() {
        let client = client.clone();
        let click_trace_len = click_traces_list.len();
        let split_idx = click_trace_len / 2;
        let idx_list: Vec<usize> = (0..split_idx).collect();
        let sampled_idx = idx_list
            .into_iter()
            .choose_multiple(rng, click_trace_sample_size_per_client);
        client_to_sample_idx_map.insert(client, sampled_idx);
    }
    client_to_sample_idx_map
}

fn eval(
    conf: &Properties,
    client_to_histogram_map: &HashMap<u32, Vec<ClickTrace>>,
    client_to_target_idx_map: &HashMap<u32, usize>,
    client_to_sample_idx_map: Option<&HashMap<u32, Vec<usize>>>,
) {
    let fields: Vec<DataFields> = conf
        .get("fields")
        .unwrap()
        .split(",")
        .map(|x| DataFields::from_str(x).unwrap())
        .collect();
    let metric = DistanceMetrics::from_str(conf.get("metric").unwrap()).unwrap();

    let is_typical_session = conf
        .get("is_typical_session")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    let result_list: Vec<(u32, u32)> = client_to_target_idx_map
        .par_iter()
        .map(|(client, target_idx)| {
            eval_step(
                is_typical_session,
                &fields,
                &metric,
                client,
                target_idx,
                &client_to_histogram_map,
                client_to_sample_idx_map,
            )
        })
        .collect();

    let mut correct_pred = 0;
    for (pred, target) in result_list.iter() {
        if pred == target {
            correct_pred += 1
        }
    }
    let accuracy: f64 = correct_pred as f64 / result_list.len() as f64;
    println!("Accuracy: {:?}", accuracy)
}

fn eval_step(
    is_typical_session: bool,
    fields: &Vec<DataFields>,
    metric: &DistanceMetrics,
    client_target: &u32,
    target_idx: &usize,
    client_to_histogram_map: &HashMap<u32, Vec<ClickTrace>>,
    client_to_sample_idx_map: Option<&HashMap<u32, Vec<usize>>>,
) -> (u32, u32) {
    let target_histogram = client_to_histogram_map
        .get(client_target)
        .unwrap()
        .get(*target_idx)
        .unwrap();

    let mut lowest_dist = std::f64::INFINITY;
    let mut client_pred = 0;

    for (client, click_traces) in client_to_histogram_map.into_iter() {
        if is_typical_session {
            let histograms = client_to_histogram_map.get(client).unwrap();

            let (website_set, code_set, location_set, category_set) =
                utils::get_unique_sets(&target_histogram, histograms);

            let vectorized_target = utils::vectorize_histogram(
                target_histogram,
                &website_set,
                &code_set,
                &location_set,
                &category_set,
            );

            let vectorized_ref = utils::compute_typical_click_trace(
                histograms,
                &website_set,
                &code_set,
                &location_set,
                &category_set,
            );

            let dist = compute_dist(fields, metric, &vectorized_target, &vectorized_ref);
            if dist < lowest_dist {
                lowest_dist = dist;
                client_pred = client.clone();
            }
        } else {
            let client_to_sample_idx_map = client_to_sample_idx_map.unwrap();
            let samples_idx = client_to_sample_idx_map.get(client).unwrap();

            let sampled_histograms: Vec<ClickTrace> = samples_idx
                .into_iter()
                .map(|idx| click_traces.get(*idx).unwrap().clone())
                .collect();

            let (website_set, code_set, location_set, category_set) =
                utils::get_unique_sets(target_histogram, &sampled_histograms);

            let vectorized_target = utils::vectorize_histogram(
                target_histogram,
                &website_set,
                &code_set,
                &location_set,
                &category_set,
            );

            for sample_histogram in sampled_histograms.into_iter() {
                let vectorized_ref = utils::vectorize_histogram(
                    &sample_histogram,
                    &website_set,
                    &code_set,
                    &location_set,
                    &category_set,
                );
                let dist = compute_dist(fields, metric, &vectorized_target, &vectorized_ref);
                if dist < lowest_dist {
                    lowest_dist = dist;
                    client_pred = client.clone();
                }
            }
        }
    }
    return (client_pred, client_target.clone());
}

// Calculate the distance between the target and the reference click trace
fn compute_dist(
    fields: &Vec<DataFields>,
    metric: &DistanceMetrics,
    target_click_trace: &ClickTraceVectorized,
    ref_click_trace: &ClickTraceVectorized,
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
        };

        let dist = match metric {
            DistanceMetrics::Euclidean => metrics::euclidean_dist(target_vector, ref_vector),
            DistanceMetrics::Manhatten => metrics::manhatten_dist(target_vector, ref_vector),
            DistanceMetrics::Cosine => metrics::consine_dist(target_vector, ref_vector),
            DistanceMetrics::Jaccard => metrics::jaccard_dist(target_vector, ref_vector),
            DistanceMetrics::Bhattacharyya => {
                metrics::bhattacharyya_dist(target_vector, ref_vector)
            }
            DistanceMetrics::KullbrackLeibler => {
                metrics::kullbrack_leibler_dist(target_vector, ref_vector)
            }
            DistanceMetrics::TotalVariation => {
                metrics::total_varation_dist(target_vector, ref_vector)
            }
            DistanceMetrics::JeffriesMatusita => {
                metrics::jeffries_matusita_dist(target_vector, ref_vector)
            }
        };
        total_dist.push(dist);
    }

    // Compute the final score by averaging the indivdual scores
    let avg_dist = total_dist.iter().sum::<f64>() / total_dist.len() as f64;
    avg_dist
}
