mod structs;
mod utils;

use core::num;
use std::collections::HashMap;
use std::error::Error;
use std::process;

use indexmap::set::IndexSet;
use rand::{Rng, seq::IteratorRandom, thread_rng};
use seal::pair::{Alignment, AlignmentSet, MemoryBacking, NeedlemanWunsch, SmithWaterman, Step};

use structs::{ClickTrace, ClickTraceVectorized, Record};

const FILE_PATH: &str = "/Users/fjohn/Documents/temp/test.csv";
const DELAY_LIMIT: f64 = 1800.0;
const CLICK_TRACE_MAX_LENGTH: usize = 100;
const CLICK_TRACE_MIN_LENGTH: usize = 1;
const SAMPLE_SIZE: usize = 2;

fn extract_from_csv() -> Result<HashMap<String, Vec<ClickTraceVectorized>>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(&FILE_PATH)?; // StreamReader?

    let mut prev_time: f64 = 0.0;
    let mut client_to_click_traces_map: HashMap<String, Vec<ClickTrace>> = HashMap::new();
    let mut click_trace_length: usize = 0;

    let mut website_set: IndexSet<String> = IndexSet::new();
    let mut code_set: IndexSet<String> = IndexSet::new();
    let mut location_set: IndexSet<String> = IndexSet::new();
    let mut category_set: IndexSet<String> = IndexSet::new();

    for result in reader.deserialize() {
        let record: Record = result?;

        if !client_to_click_traces_map.contains_key(&record.client_id) {
            client_to_click_traces_map.insert(record.client_id.clone(), Vec::with_capacity(100));
        }

        let mut click_traces_list = client_to_click_traces_map
            .get_mut(&record.client_id)
            .unwrap();

        if click_traces_list.is_empty()
            || click_traces_list.len() >= CLICK_TRACE_MAX_LENGTH
            || record.timestamp - prev_time > DELAY_LIMIT
        {
            if click_trace_length < CLICK_TRACE_MIN_LENGTH && !click_traces_list.is_empty() {
                click_traces_list.pop();
                click_trace_length = 0;
            }

            let mut click_trace = ClickTrace {
                website: HashMap::new(),
                code: HashMap::new(),
                location: HashMap::new(),
                category: HashMap::new(),
            };
            click_traces_list.push(click_trace);
        }

        let mut current_click_trace = click_traces_list.last_mut().unwrap();

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
        click_trace_length += 1;

        website_set.insert(record.website);
        code_set.insert(record.code);
        location_set.insert(record.location);
        category_set.insert(record.category);
    }

    let mut client_to_click_traces_vectorized_map: HashMap<String, Vec<ClickTraceVectorized>> =
        HashMap::new();

    for (key, click_traces_list) in client_to_click_traces_map.into_iter() {
        let mut click_traces_vectorized_list = Vec::new();
        let mut num_click_trace: usize = 0;
        for click_trace in click_traces_list.iter() {
            if num_click_trace < click_traces_list.len() / 2 {
                let click_trace_vectorized = utils::vectorize_click_trace(
                    click_trace,
                    &website_set,
                    &code_set,
                    &location_set,
                    &category_set,
            );
            click_traces_vectorized_list.push(click_trace_vectorized);
            } else {
                let half_index = click_traces_list.len() / 2;
                let mut rng = thread_rng();
                let rand_index: u32 = rng.gen_range(click_traces_list.len());
                let target_click_trace = click_traces_list.get(rand_index).unwrap();
                if num_click_trace < click_traces_list.len() / 2 {
                    let target_click_trace_vectorized = utils::vectorize_click_trace(
                        click_trace,
                        &website_set,
                        &code_set,
                        &location_set,
                        &category_set,
                );
                click_traces_vectorized_list.push(target_click_trace_vectorized);
                client_to_click_traces_vectorized_map.insert(key, click_traces_vectorized_list);
                break;
                }
            }
        num_click_trace += 1;
        }
    }
    println!("{:?}", client_to_click_traces_vectorized_map);
    Ok(client_to_click_traces_vectorized_map)
}

fn run_experiment(client_to_click_traces_map: &HashMap<String, Vec<ClickTraceVectorized>>) {
    let client_list: Vec<String> = client_to_click_traces_map.keys().cloned().collect();
    let mut rng = thread_rng();
    let sampled_clients_list = client_list.iter().choose_multiple(&mut rng, 4);

    for client in sampled_clients_list.into_iter() {
        let click_traces_list = client_to_click_traces_map.get(client).unwrap();
        if click_traces_list.len() / 2 >= SAMPLE_SIZE + 1 {
            let half_index: usize = click_traces_list.len() / 2;
            let target_click_trace = click_traces_list[half_index..].iter().choose(&mut rng);
            let observed_click_traces = click_traces_list[..half_index]
                .iter()
                .choose_multiple(&mut rng, SAMPLE_SIZE);
        }

        /*
        1. Target Session per Client
        2. Each Client: 50% Traces or Less

        Samples Clients (e.g 385 of 400k):


        */

    }

}

fn main() {
    let client_to_click_traces_vectorized_map: HashMap<String, Vec<ClickTraceVectorized>> =
        extract_from_csv().unwrap();
    run_experiment(&client_to_click_traces_vectorized_map);
}
