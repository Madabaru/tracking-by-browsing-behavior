pub mod structs;
pub mod utils;

use std::collections::HashMap;
use std::error::Error;

use rayon::prelude::*;

use rand::{rngs::StdRng, SeedableRng};
use rand::{seq::IteratorRandom, Rng};

use structs::{ClickTrace, ClickTraceVectorized, Record};

const FILE_PATH: &str = "/Users/fjohn/Documents/Masterarbeit/tracking-by-browsing-behavior/data/sanity.csv";
const DELAY_LIMIT: f64 = 1800.0;
const CLICK_TRACE_MAX_LEN: usize = 4; 
const CLICK_TRACE_MIN_LEN: usize = 3;
const SAMPLE_CLIENT_NUM: usize = 2;
const SAMPLE_SESSION_NUM: usize = 1;
const CLICK_TRACE_MIN_NUM: usize = 2 * SAMPLE_SESSION_NUM + 1;
const DIST_METRIC: &str = "euclidean";
const FIELDS: [&str; 4] = ["website", "code", "location", "category"];
const SEED: u64 = 1;

fn main() {
    // Set random seed for reproducability
    let mut rng = StdRng::seed_from_u64(SEED);

    let client_to_histogram_map: HashMap<u32, Vec<ClickTrace>> = extract_data_from_csv().unwrap();

    let client_to_target_idx_map: HashMap<u32, usize> =
        gen_test_data(&client_to_histogram_map, &mut rng, SAMPLE_CLIENT_NUM);
    println!("Test: {:?}", client_to_target_idx_map);

    let client_to_sample_idx_map: HashMap<u32, Vec<usize>> =
        get_train_data(&client_to_histogram_map, &mut rng, SAMPLE_SESSION_NUM);
    println!("Base: {:?}", client_to_sample_idx_map);

    eval(
        &client_to_histogram_map,
        &client_to_target_idx_map,
        &client_to_sample_idx_map,
    );
}

fn extract_data_from_csv() -> Result<HashMap<u32, Vec<ClickTrace>>, Box<dyn Error>> {
    let mut prev_time: f64 = 0.0;
    let mut prev_client = String::new();
    let mut click_trace_len: usize = 0;
    let mut client_id: u32 = 0;

    let mut client_to_histogram_map: HashMap<u32, Vec<ClickTrace>> = HashMap::new();

    let mut reader = csv::Reader::from_path(&FILE_PATH)?;

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_client != record.client_id && !prev_client.is_empty() {
            client_id += 1;
        }

        if !client_to_histogram_map.contains_key(&client_id) {
            client_to_histogram_map.insert(client_id, Vec::with_capacity(10));
        }

        let click_traces_list = client_to_histogram_map.get_mut(&client_id).unwrap();

        if click_traces_list.is_empty()
            || click_trace_len >= CLICK_TRACE_MAX_LEN
            || record.timestamp - prev_time >= DELAY_LIMIT
        {
            if click_trace_len < CLICK_TRACE_MIN_LEN && !click_traces_list.is_empty() {
                click_traces_list.pop();
            }

            let click_trace = ClickTrace {
                website: HashMap::new(),
                code: HashMap::new(),
                location: HashMap::new(),
                category: HashMap::new(),
            };
            click_traces_list.push(click_trace);
            click_trace_len = 0;
        }

        let current_click_trace = click_traces_list.last_mut().unwrap();

        *current_click_trace
            .website
            .entry(record.website.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .code
            .entry(record.code.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .location
            .entry(record.location.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .category
            .entry(record.category.clone())
            .or_insert(0) += 1;

        prev_time = record.timestamp;
        prev_client = record.client_id;
        click_trace_len += 1;
    }

    // Remove any client with less than the miniumum number of click traces
    client_to_histogram_map.retain(|_, value| value.len() >= CLICK_TRACE_MIN_NUM);
    Ok(client_to_histogram_map)
}

// Sample a subset of clients and a target click trace that the evaluation is based upon
fn gen_test_data<R: Rng>(
    client_to_histogram_map: &HashMap<u32, Vec<ClickTrace>>,
    rng: &mut R,
    sample_client_size: usize,
) -> HashMap<u32, usize> {
    let client_list: Vec<u32> = client_to_histogram_map.keys().cloned().collect();
    // Randomly sample clients from the client list
    let sampled_clients_list = client_list.iter().choose_multiple(rng, sample_client_size);
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
    mut rng: &mut R,
    sample_session_size: usize,
) -> HashMap<u32, Vec<usize>> {
    let mut client_to_sample_idx_map: HashMap<u32, Vec<usize>> = HashMap::new();
    for (client, click_traces_list) in client_to_histogram_map.into_iter() {
        let client = client.clone();
        let click_trace_len = click_traces_list.len();
        let split_idx = click_trace_len / 2;
        let idx_list: Vec<usize> = (0..split_idx).collect();
        let sampled_idx = idx_list
            .into_iter()
            .choose_multiple(rng, sample_session_size);
        client_to_sample_idx_map.insert(client, sampled_idx);
    }
    client_to_sample_idx_map
}

fn eval(
    client_to_histogram_map: &HashMap<u32, Vec<ClickTrace>>,
    client_to_target_idx_map: &HashMap<u32, usize>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) {
    let result_list: Vec<(u32, u32)> = client_to_target_idx_map
        .par_iter()
        .map(|(client, target_idx)| {
            eval_step(
                client,
                target_idx,
                &client_to_histogram_map,
                &client_to_sample_idx_map,
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
    client_target: &u32,
    target_idx: &usize,
    client_to_histogram_map: &HashMap<u32, Vec<ClickTrace>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) -> (u32, u32) {
    let target_histogram = client_to_histogram_map
        .get(client_target)
        .unwrap()
        .get(*target_idx)
        .unwrap();


    let mut lowest_dist = std::f64::INFINITY;
    let mut client_pred = 0;

    for (client, click_traces) in client_to_histogram_map.into_iter() {
        let samples_idx = client_to_sample_idx_map.get(client).unwrap();
        let sampled_histograms = samples_idx
            .into_iter()
            .map(|idx| click_traces.get(*idx).unwrap())
            .collect::<Vec<&ClickTrace>>();
        let (website_set, code_set, location_set, category_set) =
            utils::get_unique_sets(&target_histogram, &sampled_histograms);

        let mut vectorized_target = utils::vectorize_histogram(
            target_histogram,
            &website_set,
            &code_set,
            &location_set,
            &category_set,
        );

        for sample_histogram in sampled_histograms.into_iter() {
            let mut vectorized_ref = utils::vectorize_histogram(
                sample_histogram,
                &website_set,
                &code_set,
                &location_set,
                &category_set,
            );
            let dist = compute_dist(&vectorized_target, &vectorized_ref);
            if dist < lowest_dist {
                lowest_dist = dist;
                client_pred = client.clone();
            }
        }
    }
    println!("Predicted - Target Index:{}  Target Client:{} Pred Client:{}", target_idx, client_target, client_pred);
    (client_pred, client_target.clone())
}

// Calculate the distance between the target and the reference click trace
fn compute_dist(
    target_click_trace: &ClickTraceVectorized,
    ref_click_trace: &ClickTraceVectorized,
) -> f64 {
    // Vector to store distance scores for each data to be considered
    let mut total_dist = Vec::<f64>::with_capacity(FIELDS.len());

    // Iterate over all data fields that are considered
    for field in FIELDS.iter() {
        let (target_vector, ref_vector) = match field.as_ref() {
            "website" => (target_click_trace.website.clone(), ref_click_trace.website.clone()),
            "code" => (target_click_trace.code.clone(), ref_click_trace.code.clone()),
            "location" => (target_click_trace.location.clone(), ref_click_trace.location.clone()),
            "category" => (target_click_trace.category.clone(), ref_click_trace.category.clone()),
            x => panic!("Unknown field supplied: {}", x),
        };

        let mut dist = 0.0;

        if DIST_METRIC == "euclidean" {
            dist = utils::euclidean_dist(&target_vector, &ref_vector);
        }
        total_dist.push(dist);
    }

    // Compute the final score by averaging the indivdual scores
    let avg_dist = total_dist.iter().sum::<f64>() / total_dist.len() as f64;
    avg_dist
}
