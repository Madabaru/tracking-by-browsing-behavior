use crate::frequency::maths;
use crate::utils;

use std::collections::HashMap;

use indexmap::IndexSet;

#[derive(Debug, Clone)]
pub struct FreqClickTrace {
    pub website: HashMap<String, u32>,
    pub code: HashMap<String, u32>,
    pub location: String,
    pub category: HashMap<String, u32>,
    pub hour: Vec<u32>,
    pub day: Vec<u32>,
    pub start_time: f64,
    pub end_time: f64,
    pub click_rate: f64,
}

#[derive(Debug, Clone)]
pub struct VectFreqClickTrace {
    pub website: Vec<u32>,
    pub code: Vec<u32>,
    pub location: Vec<u32>,
    pub category: Vec<u32>,
    pub hour: Vec<u32>,
    pub day: Vec<u32>,
}

pub fn gen_typical_vect_click_trace(
    click_traces: &Vec<FreqClickTrace>,
    website_set: &IndexSet<String>,
    code_set: &IndexSet<String>,
    location_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
) -> VectFreqClickTrace {
    
    let mut website_vec = maths::zeros_u32(website_set.len());
    let mut code_vec = maths::zeros_u32(code_set.len());
    let mut location_vec = maths::zeros_u32(location_set.len());
    let mut category_vec = maths::zeros_u32(category_set.len());
    let mut hour_vec = maths::zeros_u32(24);
    let mut day_vec = maths::zeros_u32(7);

    for click_trace in click_traces.into_iter() {
        let vect_click_trace = vectorize_click_trace(
            click_trace,
            website_set,
            code_set,
            location_set,
            category_set,
        );
        website_vec = maths::add(website_vec, &vect_click_trace.website);
        code_vec = maths::add(code_vec, &vect_click_trace.code);
        location_vec = maths::add(location_vec, &vect_click_trace.location);
        category_vec = maths::add(category_vec, &vect_click_trace.category);
        day_vec = maths::add(day_vec, &vect_click_trace.day);
        hour_vec = maths::add(hour_vec, &vect_click_trace.hour);
    }

    let website_len = website_vec.len() as u32;
    website_vec.iter_mut().for_each(|a| *a /= website_len);
    let code_len = code_vec.len() as u32;
    code_vec.iter_mut().for_each(|a| *a /= code_len);
    let location_len = location_vec.len() as u32;
    location_vec.iter_mut().for_each(|a| *a /= location_len);
    let category_len = category_vec.len() as u32;
    category_vec.iter_mut().for_each(|a| *a /= category_len);
    let hour_len = category_vec.len() as u32;
    hour_vec.iter_mut().for_each(|a| *a /= hour_len);
    let day_len = category_vec.len() as u32;
    day_vec.iter_mut().for_each(|a| *a /= day_len);

    let typical_vect_click_trace = VectFreqClickTrace {
        website: website_vec,
        code: code_vec,
        location: location_vec,
        category: category_vec,
        day: day_vec,
        hour: hour_vec,
    };
    typical_vect_click_trace
}

// Transform each histogram (as a hashmap) in a click trace into a vector to speed up further computations
pub fn vectorize_click_trace(
    click_trace: &FreqClickTrace,
    website_set: &IndexSet<String>,
    code_set: &IndexSet<String>,
    location_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
) -> VectFreqClickTrace {
    let vectorized_click_trace = VectFreqClickTrace {
        website: utils::gen_vector_from_freq_map(&click_trace.website, website_set),
        code: utils::gen_vector_from_freq_map(&click_trace.code, code_set),
        location: utils::gen_vector_from_str(&click_trace.location, location_set),
        category: utils::gen_vector_from_freq_map(&click_trace.category, category_set),
        day: click_trace.hour.clone(),
        hour: click_trace.day.clone(),
    };
    vectorized_click_trace
}
