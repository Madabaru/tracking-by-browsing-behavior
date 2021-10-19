use indexmap::IndexSet;
use std::collections::HashMap;

use crate::structs::{ClickTrace, ClickTraceVectorized};

pub fn gen_freq_vec(type_to_freq_map: &HashMap<String, u64>, set: &IndexSet<String>) -> Vec<u64> {
    let mut freq_vec: Vec<u64> = vec![0; set.len()];
    for (key, value) in type_to_freq_map.into_iter() {
        freq_vec[set.get_full(key).unwrap().0] = value.clone();
    }
    return freq_vec;
}

pub fn vectorize_click_trace(
    click_trace: &ClickTrace,
    website_set: &IndexSet<String>,
    code_set: &IndexSet<String>,
    location_set: &IndexSet<String>,
    category_set: &IndexSet<String>,
) -> ClickTraceVectorized {
    let website_vector = gen_freq_vec(&click_trace.website, website_set);
    let code_vector = gen_freq_vec(&click_trace.code, code_set);
    let location_vector = gen_freq_vec(&click_trace.location, location_set);
    let category_vector = gen_freq_vec(&click_trace.category, category_set);

    let click_trace_vectorized = ClickTraceVectorized {
        website: website_vector,
        code: code_vector,
        location: location_vector,
        category: category_vector,
    };
    return click_trace_vectorized;
}
