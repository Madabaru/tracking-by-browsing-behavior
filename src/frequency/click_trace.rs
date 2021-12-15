use crate::frequency::maths;
use crate::utils;

use indexmap::IndexSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FreqClickTrace {
    pub url: HashMap<String, u32>,
    pub domain: HashMap<String, u32>,
    pub category: HashMap<String, u32>,
    pub age: String,
    pub gender: String,
    pub hour: Vec<u32>,
    pub day: Vec<u32>,
    pub start_time: f64,
    pub end_time: f64,
    pub click_rate: f64,
}

#[derive(Debug, Clone)]
pub struct VectFreqClickTrace<T> {
    pub url: Vec<T>,
    pub domain: Vec<T>,
    pub category: Vec<T>,
    pub hour: Vec<T>,
    pub day: Vec<T>,
    pub age: Vec<T>,
    pub gender: Vec<T>,
    pub click_rate: Vec<T>,
}

pub fn gen_typical_vect_click_trace(
    click_traces: &Vec<FreqClickTrace>,
    url_set: &IndexSet<String>,
    domain_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
    age_set: &IndexSet<String>,
    gender_set: &IndexSet<String>,
) -> VectFreqClickTrace<f64> {
    let mut url_vec = maths::zeros_f64(url_set.len());
    let mut domain_vec = maths::zeros_f64(domain_set.len());
    let mut category_vec = maths::zeros_f64(category_set.len());
    let mut age_vec = maths::zeros_f64(age_set.len());
    let mut gender_vec = maths::zeros_f64(gender_set.len());
    let mut hour_vec = maths::zeros_f64(24);
    let mut day_vec = maths::zeros_f64(7);
    let mut click_rate_vec = maths::zeros_f64(20);

    for click_trace in click_traces.into_iter() {
        let vect_click_trace = vectorize_click_trace(
            click_trace,
            url_set,
            domain_set,
            category_set,
            age_set,
            gender_set,
        );
        url_vec = maths::add(url_vec, &vect_click_trace.url);
        domain_vec = maths::add(domain_vec, &vect_click_trace.domain);
        category_vec = maths::add(category_vec, &vect_click_trace.category);
        day_vec = maths::add(day_vec, &vect_click_trace.day);
        hour_vec = maths::add(hour_vec, &vect_click_trace.hour);
        age_vec = maths::add(age_vec, &vect_click_trace.age);
        gender_vec = maths::add(gender_vec, &vect_click_trace.gender);
        click_rate_vec = maths::add(click_rate_vec, &vect_click_trace.click_rate);
    }

    url_vec.iter_mut().for_each(|a| *a /= click_traces.len() as f64);
    domain_vec.iter_mut().for_each(|a| *a /= click_traces.len() as f64);
    category_vec.iter_mut().for_each(|a| *a /= click_traces.len() as f64);
    hour_vec.iter_mut().for_each(|a| *a /= click_traces.len() as f64);
    day_vec.iter_mut().for_each(|a| *a /= click_traces.len() as f64);
    age_vec.iter_mut().for_each(|a| *a /= click_traces.len() as f64);
    gender_vec.iter_mut().for_each(|a| *a /= click_traces.len() as f64);
    click_rate_vec.iter_mut().for_each(|a| *a /= click_traces.len() as f64);

    let typical_vect_click_trace = VectFreqClickTrace {
        url: url_vec,
        domain: domain_vec,
        category: category_vec,
        day: day_vec,
        hour: hour_vec,
        age: age_vec,
        gender: gender_vec,
        click_rate: click_rate_vec,
    };
    typical_vect_click_trace
}

// Transform each histogram (as a hashmap) in a click trace into a vector to speed up further computations
pub fn vectorize_click_trace(
    click_trace: &FreqClickTrace,
    url_set: &IndexSet<String>,
    domain_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
    age_set: &IndexSet<String>,
    gender_set: &IndexSet<String>,
) -> VectFreqClickTrace<u32> {
    let vectorized_click_trace = VectFreqClickTrace {
        url: utils::gen_vector_from_freq_map(&click_trace.url, url_set),
        domain: utils::gen_vector_from_freq_map(&click_trace.domain, domain_set),
        category: utils::gen_vector_from_freq_map(&click_trace.category, category_set),
        age: utils::gen_vector_from_str(&click_trace.age, age_set),
        gender: utils::gen_vector_from_str(&click_trace.gender, gender_set),
        day: click_trace.day.clone(),
        hour: click_trace.hour.clone(),
        click_rate: utils::gen_vector_from_f64(click_trace.click_rate, 0.0, 2.0),
    };
    vectorized_click_trace
}
