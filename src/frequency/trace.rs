use crate::frequency::maths;
use crate::utils;

use indexmap::IndexSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FreqTrace {
    pub url: HashMap<String, u32>,
    pub domain: HashMap<String, u32>,
    pub category: HashMap<String, u32>,
    pub age: String,
    pub gender: String,
    pub hour: Vec<u32>,
    pub day: Vec<u32>,
    pub start_time: f64,
    pub end_time: f64
}

#[derive(Debug, Clone)]
pub struct VectFreqTrace<T> {
    pub url: Vec<T>,
    pub domain: Vec<T>,
    pub category: Vec<T>,
    pub hour: Vec<T>,
    pub day: Vec<T>,
    pub age: Vec<T>,
    pub gender: Vec<T>
}

/// Generates a typical (vectorized) trace from a given list of traces.
/// 
/// The distribution of values for each data field is determined by taking the average.
pub fn gen_typical_vect_trace(
    traces: &Vec<FreqTrace>,
    url_set: &IndexSet<String>,
    domain_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
    age_set: &IndexSet<String>,
    gender_set: &IndexSet<String>,
) -> VectFreqTrace<f64> {
    let mut url_vec = maths::zeros_f64(url_set.len());
    let mut domain_vec = maths::zeros_f64(domain_set.len());
    let mut category_vec = maths::zeros_f64(category_set.len());
    let mut age_vec = maths::zeros_f64(age_set.len());
    let mut gender_vec = maths::zeros_f64(gender_set.len());
    let mut hour_vec = maths::zeros_f64(24);
    let mut day_vec = maths::zeros_f64(7);

    for trace in traces.into_iter() {
        let vect_trace = vectorize_trace(
            trace,
            url_set,
            domain_set,
            category_set,
            age_set,
            gender_set,
        );
        url_vec = maths::add(url_vec, &vect_trace.url);
        domain_vec = maths::add(domain_vec, &vect_trace.domain);
        category_vec = maths::add(category_vec, &vect_trace.category);
        day_vec = maths::add(day_vec, &vect_trace.day);
        hour_vec = maths::add(hour_vec, &vect_trace.hour);
        age_vec = maths::add(age_vec, &vect_trace.age);
        gender_vec = maths::add(gender_vec, &vect_trace.gender);
    }

    url_vec.iter_mut().for_each(|a| *a /= traces.len() as f64);
    domain_vec.iter_mut().for_each(|a| *a /= traces.len() as f64);
    category_vec.iter_mut().for_each(|a| *a /= traces.len() as f64);
    hour_vec.iter_mut().for_each(|a| *a /= traces.len() as f64);
    day_vec.iter_mut().for_each(|a| *a /= traces.len() as f64);
    age_vec.iter_mut().for_each(|a| *a /= traces.len() as f64);
    gender_vec.iter_mut().for_each(|a| *a /= traces.len() as f64);

    let typical_vect_trace = VectFreqTrace {
        url: url_vec,
        domain: domain_vec,
        category: category_vec,
        day: day_vec,
        hour: hour_vec,
        age: age_vec,
        gender: gender_vec,
    };
    typical_vect_trace
}

/// Transforms each histogram (stored in a hash map) that corresponds to a trace into a fixed-size vector.
/// 
/// This tranformation to a fixed size vector greatly improves performance during the evaluation phase. 
pub fn vectorize_trace(
    trace: &FreqTrace,
    url_set: &IndexSet<String>,
    domain_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
    age_set: &IndexSet<String>,
    gender_set: &IndexSet<String>,
) -> VectFreqTrace<u32> {
    let vectorized_trace = VectFreqTrace {
        url: utils::gen_vector_from_freq_map(&trace.url, url_set),
        domain: utils::gen_vector_from_freq_map(&trace.domain, domain_set),
        category: utils::gen_vector_from_freq_map(&trace.category, category_set),
        age: utils::gen_vector_from_str(&trace.age, age_set),
        gender: utils::gen_vector_from_str(&trace.gender, gender_set),
        day: trace.day.clone(),
        hour: trace.hour.clone()
    };
    vectorized_trace
}
