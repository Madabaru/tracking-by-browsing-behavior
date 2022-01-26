use crate::cli::Config;

use indexmap::set::IndexSet;
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::{
    collections::HashMap,
    error::Error
};
use csv::{WriterBuilder};

// const OUTPUT_PATH: &str = "tmp/output";
const EVAL_PATH: &str = "tmp/evaluation";
// const THRESHOLD: f64 = 20.0;

pub fn normalize_vector(vector: &mut [f64]) {
    let norm = vector.iter().map(|x| *x * *x).sum::<f64>().sqrt();
    if norm > 0. {
        for i in vector.iter_mut() {
            *i = *i / norm;
        }
    }
}

pub fn gen_vector_from_f64(click_rate: f64, min: f64, max: f64) -> Vec<u32> {
    let diff = max - min;
    let length = (diff / 0.1) as usize;
    let idx = (click_rate / 0.1) as usize;
    let mut vector: Vec<u32> = vec![0; length];
    vector[idx] = 1;
    vector
}

pub fn gen_vector_from_freq_map(
    type_to_freq_map: &HashMap<String, u32>,
    set: &IndexSet<String>,
) -> Vec<u32> {
    let mut vector: Vec<u32> = vec![0; set.len()];
    for (key, value) in type_to_freq_map.into_iter() {
        vector[set.get_full(key).unwrap().0] = value.clone();
    }
    vector
}

pub fn gen_vector_from_str(s: &str, set: &IndexSet<String>) -> Vec<u32> {
    let mut vector: Vec<u32> = vec![0; set.len()];
    vector[set.get_full(s).unwrap().0] = 1;
    vector
}

pub fn is_target_in_top_k(client_target: &u32, tuples: &[(OrderedFloat<f64>, u32)]) -> bool {
    tuples.iter().any(|(_, b)| b == client_target)
}

// pub fn gen_pred_and_label_vec(client_target: &u32, tuples: &[(OrderedFloat<f64>, u32)]) -> (Vec<u32>, Vec<u32>){
//     let mut preds: Vec<u32> = vec![0; tuples.len()];
//     let mut labels: Vec<u32> = vec![0; tuples.len()];
//     let mut pred: u32 = 0;
//     let mut label: u32 = 0;
//     for (i, x) in tuples.iter().enumerate() {
//         if x.0.into_inner() < THRESHOLD {
//             pred = 1;
//         } else {
//             pred = 0;
//         }
//         if x.1 == *client_target {
//             label = 1;
//         } else {
//             label = 0;
//         }
//         preds[i] = pred;
//         labels[i] = label;
//     }
//     (preds, labels)
// }

pub fn get_most_freq_element<T>(vector: &[T]) -> T
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    let mut map = HashMap::new();
    for e in vector.into_iter() {
        *map.entry(e).or_insert(0) += 1;
    }
    let option = map.into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k);
    let most_repeated_ele = *option.unwrap();
    most_repeated_ele
}

#[derive(Serialize)]
struct Row {
    delay_limit: f64,
    fields: String,
    max_click_trace_len: usize,
    min_click_trace_len: usize,
    max_click_trace_duration: f64,
    max_click_rate: f64,
    min_num_click_traces: usize,
    client_sample_size: usize,
    click_trace_sample_size: usize,
    metric: String,
    path: String,
    seed: u64,
    typical: bool,
    strategy: String,
    scoring_matrix: String,
    approach: String,
    scope: String,
    top_1: f64,
    top_10: f64,
    top_10_percent: f64
}


pub fn write_to_file(config: &Config, top_1: f64, top_10: f64, top_10_percent: f64) -> Result<(), Box<Error>> {
    let mut file = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open(EVAL_PATH)
    .unwrap();

    let mut wtr = WriterBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_writer(file);
        
    wtr.serialize(Row {
        delay_limit: config.delay_limit,
        fields: format!("{:?}", &config.fields), // config.fields.iter().map(|x| DataFields::to_string(x)).collect(),
        max_click_trace_len: config.max_click_trace_len,
        min_click_trace_len: config.min_click_trace_len,
        max_click_trace_duration: config.max_click_trace_duration,
        max_click_rate: config.max_click_rate,
        min_num_click_traces: config.min_num_click_traces,
        client_sample_size: config.client_sample_size,
        click_trace_sample_size: config.click_trace_sample_size,
        metric: config.metric.to_string(),
        path: config.path.to_string(),
        seed: config.seed,
        typical: config.typical,
        strategy: config.strategy.to_string(),
        scoring_matrix: format!("{:?}", &config.scoring_matrix),  // String::from_utf8_lossy(&config.scoring_matrix),
        approach: config.approach.to_string(),
        scope: config.scope.to_string(),
        top_1: top_1,
        top_10: top_10,
        top_10_percent: top_10_percent
    })?;
    Ok(())
}

