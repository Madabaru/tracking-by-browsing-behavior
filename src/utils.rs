use std::collections::HashMap;
use std::iter::FromIterator;

use indexmap::set::IndexSet;

use ordered_float::OrderedFloat;

use crate::click_trace::{ClickTrace, VectClickTrace};
use crate::maths;

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

// Transform each histogram (as a hashmap) in a click trace into a vector to speed up further computations
pub fn vectorize_hist(
    click_trace: &ClickTrace,
    website_set: &IndexSet<String>,
    code_set: &IndexSet<String>,
    location_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
) -> VectClickTrace {
    let vectorized_click_trace = VectClickTrace {
        website: gen_vector_from_freq_map(&click_trace.website, website_set),
        code: gen_vector_from_freq_map(&click_trace.code, code_set),
        location: gen_vector_from_str(&click_trace.location, location_set),
        category: gen_vector_from_freq_map(&click_trace.category, category_set),
        day: click_trace.hour.clone(),
        hour: click_trace.day.clone(),
    };
    vectorized_click_trace
}

pub fn get_unique_sets(
    target_hist: &ClickTrace,
    sampled_hists: &Vec<ClickTrace>,
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

pub fn get_typ_click_trace(
    hists: &Vec<ClickTrace>,
    website_set: &IndexSet<String>,
    code_set: &IndexSet<String>,
    location_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
) -> VectClickTrace {
    let mut website_vec = maths::zeros_u32(website_set.len());
    let mut code_vec = maths::zeros_u32(code_set.len());
    let mut location_vec = maths::zeros_u32(location_set.len());
    let mut category_vec = maths::zeros_u32(category_set.len());
    let mut hour_vec = maths::zeros_u32(24);
    let mut day_vec = maths::zeros_u32(7);

    for hist in hists.into_iter() {
        let hist_vect = vectorize_hist(hist, website_set, code_set, location_set, category_set);
        website_vec = maths::add(website_vec, &hist_vect.website);
        code_vec = maths::add(code_vec, &hist_vect.code);
        location_vec = maths::add(location_vec, &hist_vect.location);
        category_vec = maths::add(category_vec, &hist_vect.category);
        day_vec = maths::add(day_vec, &hist_vect.day);
        hour_vec = maths::add(hour_vec, &hist_vect.hour);
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

    let typ_vectorized_click_trace = VectClickTrace {
        website: website_vec,
        code: code_vec,
        location: location_vec,
        category: category_vec,
        day: day_vec,
        hour: hour_vec,
    };
    typ_vectorized_click_trace
}

