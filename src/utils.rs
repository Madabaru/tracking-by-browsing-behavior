use crate::{cli, frequency::click_trace::FreqClickTrace};

use std::{collections::HashMap, iter::FromIterator};

use indexmap::set::IndexSet;
use ordered_float::OrderedFloat;

use seal::pair::{Alignment, AlignmentSet, InMemoryAlignmentMatrix, NeedlemanWunsch, SmithWaterman, Step, strategy};

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


// pub fn get_strategy<T: seal::pair::Strategy>(config: &cli::Config) -> T{

//     let scoring_matrix = config.scoring_matrix.clone();
//     let strategy: T;

//     if config.strategy == "NW" {
//         strategy = NeedlemanWunsch::new(scoring_matrix[0], scoring_matrix[1], scoring_matrix[2], scoring_matrix[3]);
//     } else {
//         strategy = SmithWaterman::new(scoring_matrix[0], scoring_matrix[1], scoring_matrix[2], scoring_matrix[3]);
//     }
//     strategy
// }