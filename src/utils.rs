use crate::maths;
use crate::structs::{ClickTrace, ClickTraceVectorized};

use std::collections::HashMap;
use std::iter::FromIterator;

use indexmap::set::IndexSet;

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
    click_trace_vectorized
}

pub fn get_unique_sets(
    target_histogram: &ClickTrace,
    sampled_histograms: &Vec<ClickTrace>, // &ClickTrace
) -> (
    IndexSet<String>,
    IndexSet<String>,
    IndexSet<String>,
    IndexSet<String>,
) {
    let mut website_vector: Vec<String> = target_histogram.website.keys().cloned().collect();
    let mut code_vector: Vec<String> = target_histogram.code.keys().cloned().collect();
    let mut location_vector: Vec<String> = target_histogram.location.keys().cloned().collect();
    let mut category_vector: Vec<String> = target_histogram.category.keys().cloned().collect();

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

pub fn compute_typical_click_trace(
    histograms: &Vec<ClickTrace>,
    website_set: &IndexSet<String>,
    code_set: &IndexSet<String>,
    location_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
) -> ClickTraceVectorized {
    let mut website_vector = maths::zeros_u32(website_set.len());
    let mut code_vector = maths::zeros_u32(code_set.len());
    let mut location_vector = maths::zeros_u32(location_set.len());
    let mut category_vector = maths::zeros_u32(category_set.len());

    for histogram in histograms.into_iter() {
        let histo_vectorized =
            vectorize_histogram(histogram, website_set, code_set, location_set, category_set);
        website_vector = maths::add(website_vector, &histo_vectorized.website);
        code_vector = maths::add(code_vector, &histo_vectorized.code);
        location_vector = maths::add(location_vector, &histo_vectorized.location);
        category_vector = maths::add(category_vector, &histo_vectorized.category);
    }

    let website_len = website_vector.len() as u32;
    website_vector.iter_mut().for_each(|a| *a /= website_len);
    let code_len = code_vector.len() as u32;
    code_vector.iter_mut().for_each(|a| *a /= code_len);
    let location_len = location_vector.len() as u32;
    location_vector.iter_mut().for_each(|a| *a /= location_len);
    let category_len = category_vector.len() as u32;
    category_vector.iter_mut().for_each(|a| *a /= category_len);

    let typical_click_trace_vectorized = ClickTraceVectorized {
        website: website_vector,
        code: code_vector,
        location: location_vector,
        category: category_vector,
    };
    typical_click_trace_vectorized
}
