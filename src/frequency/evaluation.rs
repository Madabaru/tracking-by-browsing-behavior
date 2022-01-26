use crate::cli;
use crate::frequency::{
    click_trace,
    click_trace::{FreqClickTrace, VectFreqClickTrace},
    metrics,
    metrics::DistanceMetric,
};
use crate::parse::DataFields;
use crate::utils;

use indexmap::IndexSet;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    iter::FromIterator,
    str::FromStr,
};

pub fn eval(
    config: &cli::Config,
    client_to_freq_map: &BTreeMap<u32, Vec<FreqClickTrace>>,
    client_to_target_idx_map: &HashMap<u32, usize>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) {
    let result_list: Vec<(bool, bool, bool)> = client_to_target_idx_map
        .par_iter()
        .map(|(client, target_idx)| {
            eval_step(
                config,
                client,
                target_idx,
                &client_to_freq_map,
                client_to_sample_idx_map,
            )
        })
        .collect();

    let mut top_1_count = 0;
    let mut top_10_count = 0;
    let mut top_10_percent_count = 0;
    for (in_top_1, in_top_10, in_top_10_percent) in result_list.iter() {
        if *in_top_1 {
            top_1_count += 1
        }
        if *in_top_10 {
            top_10_count += 1;
        }
        if *in_top_10_percent {
            top_10_percent_count += 1;
        }
    }

    let top_1: f64 = top_1_count as f64 / result_list.len() as f64;
    log::info!("Rank 1: {:?}", top_1);
    let top_10: f64 = top_10_count as f64 / result_list.len() as f64;
    log::info!("Top 10: {:?}", top_10);
    let top_10_percent: f64 = top_10_percent_count as f64 / result_list.len() as f64;
    log::info!("Top 10 Percent: {:?}", top_10_percent);
    
    // Write metrics to final evaluation file
    utils::write_to_file(config, top_1, top_10, top_10_percent);
}

fn eval_step(
    config: &cli::Config,
    client_target: &u32,
    target_idx: &usize,
    client_to_freq_map: &BTreeMap<u32, Vec<FreqClickTrace>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) -> (bool, bool, bool) {
    let metric = DistanceMetric::from_str(&config.metric).unwrap();
    let target_click_trace = client_to_freq_map
        .get(client_target)
        .unwrap()
        .get(*target_idx)
        .unwrap();

    let mut tuples: Vec<(OrderedFloat<f64>, u32)> = Vec::with_capacity(client_to_freq_map.len());

    for (client, click_traces) in client_to_freq_map.into_iter() {
        let samples_idx = client_to_sample_idx_map.get(client).unwrap();
        let sampled_click_traces: Vec<FreqClickTrace> = samples_idx
            .into_iter()
            .map(|idx| click_traces.get(*idx).unwrap().clone())
            .collect();

        let url_set = get_unique_set(target_click_trace, &sampled_click_traces, &DataFields::Url);
        let domain_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Domain,
        );
        let category_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Category,
        );
        let age_set = get_unique_set(target_click_trace, &sampled_click_traces, &DataFields::Age);
        let gender_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Gender,
        );

        let vect_target_click_trace = click_trace::vectorize_click_trace(
            target_click_trace,
            &url_set,
            &domain_set,
            &category_set,
            &age_set,
            &gender_set,
        );

        if config.typical {
            let vect_typ_ref_click_trace = click_trace::gen_typical_vect_click_trace(
                &sampled_click_traces,
                &url_set,
                &domain_set,
                &category_set,
                &age_set,
                &gender_set,
            );
            let dist = compute_dist(
                &config.fields,
                &metric,
                &vect_target_click_trace,
                &vect_typ_ref_click_trace,
            );
            tuples.push((OrderedFloat(dist), client.clone()));
        } else {
            for click_trace in sampled_click_traces.into_iter() {
                let vect_ref_click_trace = click_trace::vectorize_click_trace(
                    &click_trace,
                    &url_set,
                    &domain_set,
                    &category_set,
                    &age_set,
                    &gender_set,
                );
                let dist =
                    compute_dist(&config.fields, &metric, &vect_target_click_trace, &vect_ref_click_trace);
                tuples.push((OrderedFloat(dist), client.clone()));
            }
        }
    }
    tuples.sort_unstable_by_key(|k| k.0);
    let cutoff: usize = (0.1 * client_to_freq_map.len() as f64) as usize;
    let is_top_10_percent = utils::is_target_in_top_k(client_target, &tuples[..cutoff]);
    let is_top_10: bool = utils::is_target_in_top_k(client_target, &tuples[..1]);
    let is_top_1: bool = client_target.clone() == tuples[0].1;
    // let tuple_preds_and_labels = utils::gen_pred_and_label_vec(client_target, &tuples);
    (
        is_top_1,
        is_top_10,
        is_top_10_percent,
    )
}

// Calculate the distance between the target and the reference click trace
fn compute_dist<T, U>(
    fields: &Vec<DataFields>,
    metric: &DistanceMetric,
    target_click_trace: &VectFreqClickTrace<T>,
    ref_click_trace: &VectFreqClickTrace<U>,
) -> f64
where
    T: Clone
        + std::cmp::PartialEq
        + std::fmt::Debug
        + num_traits::ToPrimitive
        + std::cmp::PartialOrd
        + num_traits::Zero,
    U: Clone
        + std::cmp::PartialEq
        + std::fmt::Debug
        + num_traits::ToPrimitive
        + std::cmp::PartialOrd
        + num_traits::Zero,
{
    // Vector to store distance scores for each data field to be considered
    let mut total_dist = Vec::<f64>::with_capacity(fields.len());

    // Iterate over all data fields that are considered
    for field in fields.into_iter() {
        let (target_vector, ref_vector) = match field {
            DataFields::Url => (target_click_trace.url.clone(), ref_click_trace.url.clone()),
            DataFields::Domain => (
                target_click_trace.domain.clone(),
                ref_click_trace.domain.clone(),
            ),
            DataFields::Category => (
                target_click_trace.category.clone(),
                ref_click_trace.category.clone(),
            ),
            DataFields::Day => (target_click_trace.day.clone(), ref_click_trace.day.clone()),
            DataFields::Hour => (
                target_click_trace.hour.clone(),
                ref_click_trace.hour.clone(),
            ),
            DataFields::Gender => (
                target_click_trace.gender.clone(),
                ref_click_trace.gender.clone(),
            ),
            DataFields::Age => (target_click_trace.age.clone(), ref_click_trace.age.clone()),
            DataFields::ClickRate => (
                target_click_trace.click_rate.clone(),
                ref_click_trace.click_rate.clone(),
            ),
            _ => panic!("Error: unknown data field supplied: {}", field),
        };

        let dist = match metric {
            DistanceMetric::Euclidean => metrics::euclidean_dist(target_vector, ref_vector),
            DistanceMetric::Manhattan => metrics::manhattan_dist(target_vector, ref_vector),
            DistanceMetric::Cosine => metrics::consine_dist(target_vector, ref_vector),
            DistanceMetric::NonIntersection => metrics::non_intersection_dist(target_vector, ref_vector),
            DistanceMetric::Bhattacharyya => metrics::bhattacharyya_dist(target_vector, ref_vector),
            DistanceMetric::KullbrackLeibler => metrics::kullbrack_leibler_dist(target_vector, ref_vector),
            DistanceMetric::TotalVariation => metrics::total_variation_dist(target_vector, ref_vector),
            DistanceMetric::JeffriesMatusita => metrics::jeffries_dist(target_vector, ref_vector),
            DistanceMetric::ChiSquared => metrics::chi_squared_dist(target_vector, ref_vector),
        };
        total_dist.push(dist);
    }

    // Compute the final score by averaging the indivdual scores
    let avg_dist = total_dist.iter().sum::<f64>() / total_dist.len() as f64;
    avg_dist
}

pub fn get_unique_set(
    target_click_trace: &FreqClickTrace,
    sampled_click_traces: &Vec<FreqClickTrace>,
    field: &DataFields,
) -> IndexSet<String> {
    let mut vector: Vec<String> = match field {
        DataFields::Url => target_click_trace.url.keys().cloned().collect(),
        DataFields::Domain => target_click_trace.domain.keys().cloned().collect(),
        DataFields::Category => target_click_trace.category.keys().cloned().collect(),
        DataFields::Age => Vec::from([target_click_trace.age.clone()]),
        DataFields::Gender => Vec::from([target_click_trace.gender.clone()]),
        _ => panic!("Error: unknown data field supplied: {}", field),
    };

    for click_trace in sampled_click_traces.into_iter() {
        match field {
            DataFields::Url => vector.extend(click_trace.url.keys().cloned()),
            DataFields::Domain => vector.extend(click_trace.domain.keys().cloned()),
            DataFields::Category => vector.extend(click_trace.category.keys().cloned()),
            DataFields::Age => vector.push(click_trace.age.clone()),
            DataFields::Gender => vector.push(click_trace.gender.clone()),
            _ => panic!("Error: unknown data field supplied: {}", field),
        }
    }
    let set: IndexSet<String> = IndexSet::from_iter(vector);
    set
}
