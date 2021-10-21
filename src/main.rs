pub mod structs;
pub mod utils;

use std::collections::HashMap;
use std::error::Error;

use rayon::prelude::*;

use indexmap::set::IndexSet;

use ndarray::{array, Array};

use rand::{rngs::StdRng, SeedableRng};
use rand::{seq::IteratorRandom, Rng};

// use seal::pair::{SmithWaterman, NeedlemanWunsch, AlignmentSet, Alignment, Step};

use structs::{ClickTrace, Record};

const FILE_PATH: &str = "/Users/fjohn/Documents/temp/test.csv";
const DELAY_LIMIT: f64 = 1800.0;
const CLICK_TRACE_MAX_LEN: usize = 100;
const CLICK_TRACE_MIN_LEN: usize = 1;
const SAMPLE_CLIENT_NUM: usize = 10;
const SAMPLE_SESSION_NUM: usize = 3;
const CLICK_TRACE_MIN_NUM: usize = 2 * SAMPLE_SESSION_NUM + 1;
const DIST_METRIC: &str = "euclidean";
const FIELDS: [&str; 4] = ["website", "code", "location", "category"];
const SEED: u64 = 1;

fn main() {
    // Set random seed for reproducability
    let mut rng = StdRng::seed_from_u64(SEED);

    let client_to_click_traces_map: HashMap<u32, Vec<ClickTrace>> =
        extract_data_from_csv().unwrap();

    let client_to_target_idx_map: HashMap<u32, usize> =
        gen_test_data(&client_to_click_traces_map, &mut rng, SAMPLE_CLIENT_NUM);

    let client_to_sample_idx_map: HashMap<u32, Vec<usize>> =
        get_train_data(&client_to_click_traces_map, &mut rng, SAMPLE_SESSION_NUM);

    eval(
        &client_to_click_traces_map,
        &client_to_target_idx_map,
        &client_to_sample_idx_map,
    );
}

fn extract_data_from_csv() -> Result<HashMap<u32, Vec<ClickTrace>>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(&FILE_PATH)?;
    let mut website_set: IndexSet<String> = IndexSet::new();
    let mut code_set: IndexSet<String> = IndexSet::new();
    let mut location_set: IndexSet<String> = IndexSet::new();
    let mut category_set: IndexSet<String> = IndexSet::new();

    // Retrieve number of unique values for all data fields
    for result in reader.deserialize() {
        let record: Record = result?;
        website_set.insert(record.website);
        code_set.insert(record.code);
        location_set.insert(record.location);
        category_set.insert(record.category);
    }

    let mut prev_time: f64 = 0.0;
    let mut prev_client = String::new();
    let mut client_to_click_traces_map = HashMap::<u32, Vec<ClickTrace>>::new();
    let mut click_traces_list = Vec::<ClickTrace>::new();
    let mut click_trace_len: usize = 0;
    let mut client_id: u32 = 0;

    let mut reader = csv::Reader::from_path(&FILE_PATH)?;

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_client != record.client_id && !prev_client.is_empty() {
            if click_traces_list.len() >= CLICK_TRACE_MIN_NUM {
                client_to_click_traces_map.insert(client_id, click_traces_list.clone());
                client_id += 1;
                click_traces_list.clear();
            }
        }

        if click_traces_list.is_empty()
            || click_traces_list.len() >= CLICK_TRACE_MAX_LEN
            || record.timestamp - prev_time > DELAY_LIMIT
        {
            if click_trace_len < CLICK_TRACE_MIN_LEN && !click_traces_list.is_empty() {
                click_traces_list.pop();
                click_trace_len = 0;
            }

            let click_trace = ClickTrace {
                website: utils::zeros(code_set.len()),
                code: utils::zeros(code_set.len()),
                location: utils::zeros(location_set.len()),
                category: utils::zeros(category_set.len()),
            };
            click_traces_list.push(click_trace);
        }

        let current_click_trace = click_traces_list.last_mut().unwrap();

        current_click_trace.website[website_set.get_full(&record.website).unwrap().0] += 1;
        current_click_trace.code[code_set.get_full(&record.code).unwrap().0] += 1;
        current_click_trace.location[location_set.get_full(&record.location).unwrap().0] += 1;
        current_click_trace.category[category_set.get_full(&record.category).unwrap().0] += 1;

        prev_time = record.timestamp;
        prev_client = record.client_id;
        click_trace_len += 1;
    }

    // println!("{:?}", client_to_click_traces_map);
    Ok(client_to_click_traces_map)
}

fn gen_test_data<R: Rng>(
    client_to_click_traces_map: &HashMap<u32, Vec<ClickTrace>>,
    rng: &mut R,
    sample_client_size: usize,
) -> HashMap<u32, usize> {
    // Sample a subset of clients and a target click trace that the evaluation is based upon
    let client_list: Vec<u32> = client_to_click_traces_map.keys().cloned().collect();
    let sampled_clients_list = client_list.iter().choose_multiple(rng, sample_client_size);

    let mut client_to_target_idx_map = HashMap::<u32, usize>::new();

    for client in sampled_clients_list.into_iter() {
        let click_traces_list = &client_to_click_traces_map.get(client).unwrap();
        let click_trace_len = click_traces_list.len();
        let split_idx = click_trace_len / 2;
        let rand_target_idx: usize = rng.gen_range(split_idx..click_trace_len);
        client_to_target_idx_map.insert(client.clone(), rand_target_idx);
    }
    return client_to_target_idx_map;
}

fn get_train_data<R: Rng>(
    client_to_click_traces_map: &HashMap<u32, Vec<ClickTrace>>,
    mut rng: &mut R,
    sample_session_size: usize,
) -> HashMap<u32, Vec<usize>> {
    // Sample click traces for each client and store sample indices in map
    let mut client_to_sample_idx_map: HashMap<u32, Vec<usize>> = HashMap::new();
    for (client, click_traces_list) in client_to_click_traces_map.into_iter() {
        let len = click_traces_list.len();
        let split_idx = len / 2;
        let idx_list: Vec<usize> = (0..split_idx).collect();
        let sampled_idx = idx_list
            .into_iter()
            .choose_multiple(rng, sample_session_size);
        client_to_sample_idx_map.insert(client.clone(), sampled_idx);
    }
    client_to_sample_idx_map
}

fn eval(
    client_to_click_traces_map: &HashMap<u32, Vec<ClickTrace>>,
    client_to_target_idx_map: &HashMap<u32, usize>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) {

    let result_list: Vec<(u32, u32)> = client_to_target_idx_map
        .par_iter()
        .map(|(client, target_idx)| eval_step(client, target_idx, client_to_click_traces_map, client_to_sample_idx_map)).collect();
        
    let mut correct_pred = 0;
    for (pred, target) in result_list.iter() {
        if pred == target {
            correct_pred += 1
        }
    }
    let accuracy : f64 = correct_pred as f64 / result_list.len() as f64;
    println!("Accuracy: {:?}", accuracy)
}

fn eval_step(
    client_target: &u32,
    target_index: &usize,
    client_to_click_traces_map: &HashMap<u32, Vec<ClickTrace>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) -> (u32, u32) {
    let target_index = target_index.clone();
    let target_click_trace = client_to_click_traces_map
        .get(client_target)
        .unwrap()
        .get(target_index)
        .unwrap();
    let mut lowest_dist = std::f64::INFINITY;
    let mut client_pred = 0;

    for (client, click_traces) in client_to_click_traces_map.into_iter() {
        let samples_idx = client_to_sample_idx_map.get(client).unwrap();
        for idx in samples_idx.iter() {
            let idx = idx.clone();
            let ref_click_trace = click_traces.get(idx).unwrap();
            let dist = compute_dist(target_click_trace, ref_click_trace);
            if dist < lowest_dist {
                lowest_dist = dist;
                client_pred = client.clone();
            }
        }
    }
    (client_pred, client_target.clone())
}

// Calculate the distance between the target and the reference click trace
fn compute_dist(target_click_trace: &ClickTrace, ref_click_trace: &ClickTrace) -> f64 {
    // Vector to store distance scores for each data to be consiedered
    let mut total_dist = Vec::<f64>::with_capacity(FIELDS.len());

    // Iterate over all data fields that are consiedered
    for field in FIELDS.iter() {
        let (target_vector, ref_vector) = match field.as_ref() {
            "website" => (
                target_click_trace.website.as_ref(),
                ref_click_trace.website.as_ref(),
            ),
            "code" => (
                target_click_trace.code.as_ref(),
                ref_click_trace.code.as_ref(),
            ),
            "location" => (
                target_click_trace.location.as_ref(),
                ref_click_trace.location.as_ref(),
            ),
            "category" => (
                target_click_trace.category.as_ref(),
                ref_click_trace.category.as_ref(),
            ),
            x => panic!("Unknown field supplied: {}", x),
        };

        let mut dist= 0.0;

        if DIST_METRIC == "euclidean" {
            dist = utils::euclidean_dist(&target_vector, &ref_vector);
        }
        total_dist.push(dist);
    }
    // Compute the final score by averaging the indivdual scores
    let avg_dist = total_dist.iter().sum::<f64>() / total_dist.len() as f64;
    avg_dist
}


