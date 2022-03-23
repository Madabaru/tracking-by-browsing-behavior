use crate::cli::Config;

use csv::WriterBuilder;
use indexmap::set::IndexSet;
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::{collections::HashMap, error::Error};

const EVAL_PATH: &str = "tmp/evaluation";

// Normalizes the values of a given vector
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

// Generates a vector of fixed size from a given frequency map. The size depends on the size set of unique values.
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

// Flattens a nested vector 
pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

pub fn gen_vector_from_str(s: &str, set: &IndexSet<String>) -> Vec<u32> {
    let mut vector: Vec<u32> = vec![0; set.len()];
    vector[set.get_full(s).unwrap().0] = 1;
    vector
}

pub fn is_target_in_top_k(client_target: &u32, tuples: &[(u32, OrderedFloat<f64>)]) -> bool {
    tuples.iter().any(|(a, _)| a == client_target)
}

// Returns the most frequent element in a given vector of values. The values can be of arbitrary type.
pub fn get_most_freq_element<T>(vector: &[T]) -> T
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    let mut map = HashMap::new();
    for e in vector.into_iter() {
        *map.entry(e).or_insert(0) += 1;
    }
    let option = map.into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k);
    let most_frequent_ele = *option.unwrap();
    most_frequent_ele
}

// Calculates the mean for a vector of values.
pub fn mean(data: &[f64]) -> f64 {
    let sum = data.iter().sum::<f64>();
    let count = data.len();
    let mean = sum / count as f64;
    mean
}

// Calculates the standard deviation for a vector of values.
pub fn std_deviation(data: &[f64]) -> f64 {
    let data_mean = mean(data);
    let count = data.len();
    let variance = data
        .iter()
        .map(|value| {
            let diff = data_mean - (*value as f64);
            diff * diff
        })
        .sum::<f64>()
        / count as f64;
    variance.sqrt()
}

pub fn is_significant(tuples: &[(u32, OrderedFloat<f64>)]) -> bool {
    let mut significant = false;
    let diff_1st_to_2nd = tuples.get(0).unwrap().1 - tuples.get(1).unwrap().1;
    let diff_2nd_to_3rd = tuples.get(1).unwrap().1 - tuples.get(2).unwrap().1;
    let siginificance_level: OrderedFloat<f64> = OrderedFloat(1.0) - (diff_2nd_to_3rd / diff_1st_to_2nd);
    if siginificance_level >= OrderedFloat(0.75) {
        significant = true;
    }
    significant
}

// Calculates the confidence interval.
//
// Calculates and returns the confidence interval in form of a tuple with lower and
// upper bound for a mean estimate.
// The default confidence level is 0.95 using a z-value= 1.96.
// pub fn get_confidence_interval(data: &[f64]) -> (f64, f64) {
//     let sample_size = data.len() as f64;
//     let mean = mean(data);
//     let std = std_deviation(data);
//     let lower_bound = mean - 1.96 * (std / f64::sqrt(sample_size));
//     let upper_bound = mean - 1.96 * (std / f64::sqrt(sample_size));
//     (lower_bound, upper_bound)
// }

#[derive(Serialize)]
struct Row {
    delay_limit: f64,
    max_click_trace_len: usize,
    min_click_trace_len: usize,
    max_click_trace_duration: f64,
    max_click_rate: f64,
    min_num_click_traces: usize,
    path: String,
    seed: u64,
    client_sample_size: usize,
    click_trace_sample_size: usize,
    target_click_trace_sample_size: usize,
    approach: String,
    fields: String,
    typical: bool,
    dependent: bool,
    multiple: bool, 
    metric: String,
    strategy: String,
    scoring_matrix: String,
    scope: String,
    top_1: f64,
    top_1_std: f64,
    top_10: f64,
    top_10_std: f64,
    top_10_percent: f64,
    top_10_percent_std: f64,
}

// Writes the performance scores as well as the configuration to file
pub fn write_to_file(
    config: &Config,
    top_1: f64,
    top_1_std: f64,
    top_10: f64,
    top_10_std: f64,
    top_10_percent: f64,
    top_10_percent_std: f64,
) -> Result<(), Box<dyn Error>> {
    let file = std::fs::OpenOptions::new()
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
        max_click_trace_len: config.max_click_trace_len,
        min_click_trace_len: config.min_click_trace_len,
        max_click_trace_duration: config.max_click_trace_duration,
        max_click_rate: config.max_click_rate,
        min_num_click_traces: config.min_num_click_traces,
        path: config.path.to_string(),
        seed: config.seed,
        approach: config.approach.to_string(),
        client_sample_size: config.client_sample_size,
        click_trace_sample_size: config.click_trace_sample_size,
        target_click_trace_sample_size: config.target_click_trace_sample_size,
        fields: format!("{:?}", &config.fields),
        typical: config.typical,
        dependent: config.dependent,
        multiple: config.multiple,
        metric: config.metric.to_string(),
        strategy: config.strategy.to_string(),
        scoring_matrix: format!("{:?}", &config.scoring_matrix),
        scope: config.scope.to_string(),
        top_1: top_1,
        top_1_std: top_1_std,
        top_10: top_10,
        top_10_std: top_10_std,
        top_10_percent: top_10_percent,
        top_10_percent_std: top_10_percent_std,
    })?;
    Ok(())
}
