
use crate::structs::{ClickTrace, ClickTraceVectorized};

use std::collections::HashMap;
use std::iter::FromIterator;

use ndarray::{Array, Array1, ArrayBase, ArrayView1};
use indexmap::set::IndexSet;

// Create vector of zeros 
pub fn zeros(size: usize) -> Vec<i32> {
    vec![0; size]
}

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e/norm);
    x
}

fn convert(vec: &[u32]) -> Vec<f64> {
    let norm_vec: Vec<f64> = vec.into_iter().map(|x| f64::from(*x)).collect();
    norm_vec
}

pub fn euclidean_dist(target_vec: &Vec<u32>, ref_vec: &Vec<u32>) -> f64 {
    let target_vec: Vec<f64> = convert(&target_vec);
    let ref_vec: Vec<f64> = convert(&ref_vec);
    let target_arr= Array::from(target_vec);
    let ref_arr = Array::from(ref_vec);
    let target_arr = normalize(target_arr);
    let ref_arr = normalize(ref_arr);
    let diff = target_arr - ref_arr;
    let pow = diff.mapv(|diff: f64| diff.powi(2));
    let sum: f64= pow.sum().into();
    sum.sqrt()
}

pub fn vectorize_hist() {

}

pub fn gen_vector(type_to_freq_map: &HashMap<String, u32>, set: &IndexSet<String>) -> Vec<u32> {
    let mut vector: Vec<u32> = vec![0; set.len()];
    for (key, value) in type_to_freq_map.into_iter() {
        vector[set.get_full(key).unwrap().0] = value.clone();
    }
    vector
}


pub fn vectorize_histogram(
    click_trace: &ClickTrace,
    website_set: &IndexSet<String>,
    code_set: &IndexSet<String>,
    location_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
) -> ClickTraceVectorized {
    let website_vector = gen_vector(&click_trace.website, website_set);
    let code_vector = gen_vector(&click_trace.code, code_set);
    let location_vector = gen_vector(&click_trace.location, location_set);
    let category_vector = gen_vector(&click_trace.category, category_set);

    let click_trace_vectorized = ClickTraceVectorized {
        website: website_vector,
        code: code_vector,
        location: location_vector,
        category: category_vector,
    };
    return click_trace_vectorized;
}


pub fn get_unique_sets(target_histogram: &ClickTrace, sampled_histograms: &Vec<&ClickTrace>) -> (IndexSet<String>, IndexSet<String>, IndexSet<String>, IndexSet<String>) {

    let mut website_vector: Vec<String>= target_histogram.website.keys().cloned().collect();
    let mut code_vector: Vec<String>= target_histogram.code.keys().cloned().collect();
    let mut location_vector: Vec<String>= target_histogram.location.keys().cloned().collect();
    let mut category_vector: Vec<String>= target_histogram.category.keys().cloned().collect();

    for histogram in sampled_histograms.into_iter() {
        website_vector.extend(histogram.website.keys().cloned());
        code_vector.extend(histogram.code.keys().cloned());
        location_vector.extend(histogram.location.keys().cloned());
        category_vector.extend(histogram.category.keys().cloned());
    }

    let website_set: IndexSet<String> = IndexSet::from_iter(website_vector);
    let code_set: IndexSet<String> = IndexSet::from_iter(code_vector);
    let location_set: IndexSet<String> = IndexSet::from_iter(location_vector);
    let category_set: IndexSet<String> = IndexSet::from_iter(category_vector);

    (website_set, code_set, location_set, category_set)
}