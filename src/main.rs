mod structs;
mod utils;

use std::collections::HashMap;
use std::error::Error;

use indexmap::set::IndexSet;

use rand::distributions::{Distribution, Uniform};
use rand::{rngs::StdRng, SeedableRng};
use rand::{seq::IteratorRandom, Rng};

// use seal::pair::{Alignment, AlignmentSet, MemoryBacking, NeedlemanWunsch, SmithWaterman, Step};

use structs::{ClickTrace, Record};

const FILE_PATH: &str = "/Users/fjohn/Documents/temp/test.csv";
const DELAY_LIMIT: f64 = 1800.0;
const CLICK_TRACE_MAX_LEN: usize = 100;
const CLICK_TRACE_MIN_LEN: usize = 1;
const SAMPLE_CLIENT_NUM: usize = 2;
const SAMPLE_SESSION_NUM: usize = 2;
const CLICK_TRACE_MIN_NUM: usize = 2 * SAMPLE_SESSION_NUM + 1;
const SEED: u64 = 0;

fn extract_data_from_csv() -> Result<HashMap<String, Vec<ClickTrace>>, Box<dyn Error>> {
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
    let mut client_to_click_traces_map: HashMap<String, Vec<ClickTrace>> = HashMap::new();
    let mut click_trace_len: usize = 0;

    let mut reader = csv::Reader::from_path(&FILE_PATH)?;

    for result in reader.deserialize() {
        let record: Record = result?;

        if !client_to_click_traces_map.contains_key(&record.client_id) {
            client_to_click_traces_map.insert(record.client_id.clone(), Vec::new());
        }

        let click_traces_list = client_to_click_traces_map
            .get_mut(&record.client_id)
            .unwrap();

        if click_traces_list.is_empty()
            || click_traces_list.len() >= CLICK_TRACE_MAX_LEN
            || record.timestamp - prev_time > DELAY_LIMIT
        {
            if click_trace_len < CLICK_TRACE_MIN_LEN && !click_traces_list.is_empty() {
                click_traces_list.pop();
                click_trace_len = 0;
            }

            let click_trace = ClickTrace {
                website: utils::zeros(website_set.len()),
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
        click_trace_len += 1;
    }

    Ok(client_to_click_traces_map)
}

fn gen_test_data<R: Rng>(
    client_to_click_traces_map: &HashMap<String, Vec<ClickTrace>>,
    rng: &mut R,
    sample_client_size: usize,
) -> HashMap<String, usize> {
    // Sample a subset of clients and a target click trace that the evaluation is based upon
    let client_list: Vec<String> = client_to_click_traces_map.keys().cloned().collect();
    let sampled_clients_list = client_list.iter().choose_multiple(rng, sample_client_size);

    let mut client_to_target_click_trace_map: HashMap<String, usize> = HashMap::new();

    for client in sampled_clients_list.into_iter() {
        let click_traces_list = &client_to_click_traces_map.get(client).unwrap();
        let click_trace_len = click_traces_list.len();
        let split_idx = click_trace_len / 2;
        let rand_target_idx: usize = rng.gen_range(split_idx..click_trace_len);
        client_to_target_click_trace_map.insert(client.clone(), rand_target_idx);
    }
    return client_to_target_click_trace_map;
}

fn get_train_data<R: Rng>(
    client_to_click_traces_map: &HashMap<String, Vec<ClickTrace>>,
    mut rng: &mut R,
    sample_session_size: usize,
) -> HashMap<String, Vec<usize>> {
    // Sample click traces for each client and store sample indices in map
    let mut client_to_sample_idx_map: HashMap<String, Vec<usize>> = HashMap::new();
    for (client_id, click_traces_list) in client_to_click_traces_map.into_iter() {
        let len = click_traces_list.len();
        let split_idx = len / 2;
        let distr = Uniform::new(0, split_idx);
        let sampled_idx: Vec<usize> = distr
            .sample_iter(&mut rng)
            .take(sample_session_size)
            .collect();
        client_to_sample_idx_map.insert(client_id.to_string(), sampled_idx);
    }
    return client_to_sample_idx_map;
}

fn main() {
    // Set random seed for reproducability
    let mut rng = StdRng::seed_from_u64(SEED);

    let client_to_click_traces_map: HashMap<String, Vec<ClickTrace>> =
        extract_data_from_csv().unwrap();

    let client_to_target_click_trace_map: HashMap<String, usize> =
        gen_test_data(&client_to_click_traces_map, &mut rng, SAMPLE_CLIENT_NUM);
    println!("{:?}", client_to_target_click_trace_map);

    let client_to_sample_idx_map: HashMap<String, Vec<usize>> =
        get_train_data(&client_to_click_traces_map, &mut rng, SAMPLE_SESSION_NUM);

    println!("{:?}", client_to_sample_idx_map);
}
