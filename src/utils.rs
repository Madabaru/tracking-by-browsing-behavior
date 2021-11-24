use crate::cli::Config;
use crate::frequency::click_trace::FreqClickTrace;

use indexmap::set::IndexSet;
use ordered_float::OrderedFloat;
use std::io::{prelude::*, BufWriter};
use std::{collections::HashMap, iter::FromIterator};

const OUTPUT_PATH: &str = "tmp/output";
const EVAL_PATH: &str = "tmp/evaluation";

pub fn normalize_vector(vector: &mut [f64]) {
    let norm = vector.iter().map(|x| *x * *x).sum::<f64>().sqrt();
    if norm > 0. {
        for i in vector.iter_mut() {
            *i = *i / norm;
        }
    }
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

pub fn get_unique_sets(
    target_hist: &FreqClickTrace,
    sampled_hists: &Vec<FreqClickTrace>,
) -> (
    IndexSet<String>,
    IndexSet<String>,
    IndexSet<String>,
    IndexSet<String>,
) {
    let mut website_vec: Vec<String> = target_hist.website.keys().cloned().collect();
    let mut code_vec: Vec<String> = target_hist.code.keys().cloned().collect();
    let mut category_vec: Vec<String> = target_hist.category.keys().cloned().collect();
    let mut location_vec: Vec<String> = Vec::from([target_hist.location.clone()]);

    for hist in sampled_hists.into_iter() {
        website_vec.extend(hist.website.keys().cloned());
        code_vec.extend(hist.code.keys().cloned());
        category_vec.extend(hist.category.keys().cloned());
        location_vec.push(hist.location.clone());
    }

    let website_set: IndexSet<String> = IndexSet::from_iter(website_vec);
    let code_set: IndexSet<String> = IndexSet::from_iter(code_vec);
    let location_set: IndexSet<String> = IndexSet::from_iter(location_vec);
    let category_set: IndexSet<String> = IndexSet::from_iter(category_vec);

    (website_set, code_set, location_set, category_set)
}

pub fn write_to_output_file(tuple_list: Vec<(u32, u32, bool, bool)>) {
    let file = std::fs::File::create(OUTPUT_PATH).unwrap();
    let mut writer = BufWriter::new(&file);
    for i in tuple_list {
        write!(writer, "{},{} \n", i.0, i.1).expect("Unable to write to output file.");
    }
}

pub fn write_to_eval_file(config: &Config, top_10: f64, top_10_percent: f64) {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(EVAL_PATH)
        .unwrap();
    write!(
        file,
        "--------------\nExperiment: {:?}\nTop 10: {}\nTop 10 Percent: {}\n",
        config, top_10, top_10_percent
    )
    .expect("Unable to write to evaluation file.");
}
