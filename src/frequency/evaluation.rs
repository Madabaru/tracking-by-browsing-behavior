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
    let result_list: Vec<(u32, u32, bool, bool)> = client_to_target_idx_map
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

    let mut correct_pred = 0;
    let mut top_10_count = 0;
    let mut top_10_percent_count = 0;
    for (pred, target, in_top_10, in_top_10_percent) in result_list.iter() {
        if pred == target {
            correct_pred += 1
        }
        if *in_top_10 {
            top_10_count += 1;
        }
        if *in_top_10_percent {
            top_10_percent_count += 1;
        }
    }

    let accuracy: f64 = correct_pred as f64 / result_list.len() as f64;
    log::info!("Rank 1: {:?}", accuracy);
    let top_10: f64 = top_10_count as f64 / result_list.len() as f64;
    log::info!("Top 10: {:?}", top_10);
    let top_10_percent: f64 = top_10_percent_count as f64 / result_list.len() as f64;
    log::info!("Top 10 Percent: {:?}", top_10_percent);

    // Write result to output file for further processing in python
    utils::write_to_output(result_list);
    // Write metrics to final evaluation file
    utils::write_to_eval(config, top_10, top_10_percent);
}

fn eval_step(
    config: &cli::Config,
    client_target: &u32,
    target_idx: &usize,
    client_to_freq_map: &BTreeMap<u32, Vec<FreqClickTrace>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) -> (u32, u32, bool, bool) {
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

        let website_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Website,
        );
        let code_set = get_unique_set(target_click_trace, &sampled_click_traces, &DataFields::Code);
        let location_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Location,
        );
        let category_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Category,
        );

        let vectorized_target = click_trace::vectorize_click_trace(
            target_click_trace,
            &website_set,
            &code_set,
            &location_set,
            &category_set,
        );

        if config.typical {
            let vect_typ_click_trace = click_trace::gen_typical_vect_click_trace(
                &sampled_click_traces,
                &website_set,
                &code_set,
                &location_set,
                &category_set,
            );
            let dist = compute_dist(
                &config.fields,
                &metric,
                &vectorized_target,
                &vect_typ_click_trace,
            );
            tuples.push((OrderedFloat(dist), client.clone()));
        } else {
            for sample_hist in sampled_click_traces.into_iter() {
                let vectorized_ref = click_trace::vectorize_click_trace(
                    &sample_hist,
                    &website_set,
                    &code_set,
                    &location_set,
                    &category_set,
                );
                let dist =
                    compute_dist(&config.fields, &metric, &vectorized_target, &vectorized_ref);
                tuples.push((OrderedFloat(dist), client.clone()));
            }
        }
    }
    tuples.sort_unstable_by_key(|k| k.0);
    let cutoff: usize = (0.1 * client_to_freq_map.len() as f64) as usize;
    let is_top_10_percent = utils::is_target_in_top_k(client_target, &tuples[..cutoff]);
    let is_top_10: bool = utils::is_target_in_top_k(client_target, &tuples[..1]);
    (
        client_target.clone(),
        tuples[0].1,
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
            DataFields::Website => (
                target_click_trace.website.clone(),
                ref_click_trace.website.clone(),
            ),
            DataFields::Code => (
                target_click_trace.code.clone(),
                ref_click_trace.code.clone(),
            ),
            DataFields::Location => (
                target_click_trace.location.clone(),
                ref_click_trace.location.clone(),
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
        };

        let dist = match metric {
            DistanceMetric::Euclidean => metrics::euclidean_dist(target_vector, ref_vector),
            DistanceMetric::Manhatten => metrics::manhatten_dist(target_vector, ref_vector),
            DistanceMetric::Cosine => metrics::consine_dist(target_vector, ref_vector),
            DistanceMetric::Jaccard => metrics::jaccard_dist(target_vector, ref_vector),
            // DistanceMetric::Jaccard => todo!(),
            DistanceMetric::Bhattacharyya => metrics::bhattacharyya_dist(target_vector, ref_vector),
            DistanceMetric::KullbrackLeibler => metrics::kl_dist(target_vector, ref_vector),
            DistanceMetric::TotalVariation => metrics::total_var_dist(target_vector, ref_vector),
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
        DataFields::Website => target_click_trace.website.keys().cloned().collect(),
        DataFields::Code => target_click_trace.code.keys().cloned().collect(),
        DataFields::Category => target_click_trace.category.keys().cloned().collect(),
        DataFields::Location => Vec::from([target_click_trace.location.clone()]),
        _ => panic!("Error: unknown data field supplied: {}", field),
    };

    for click_trace in sampled_click_traces.into_iter() {
        match field {
            DataFields::Website => vector.extend(click_trace.website.keys().cloned()),
            DataFields::Code => vector.extend(click_trace.code.keys().cloned()),
            DataFields::Category => vector.extend(click_trace.category.keys().cloned()),
            DataFields::Location => vector.push(click_trace.location.clone()),
            _ => panic!("Error: unknown data field supplied: {}", field),
        }
    }
    let set: IndexSet<String> = IndexSet::from_iter(vector);
    set
}
