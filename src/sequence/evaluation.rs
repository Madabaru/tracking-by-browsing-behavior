use crate::cli;
use crate::parse::DataFields;
use crate::sequence::click_trace::SeqClickTrace;
use crate::utils;

use std::{
    cmp::Reverse,
    collections::HashMap
};
use seal::pair::{AlignmentSet, InMemoryAlignmentMatrix, NeedlemanWunsch, SmithWaterman};
use rayon::prelude::*;
use ordered_float::OrderedFloat;

pub fn eval(
    config: &cli::Config,
    client_to_hist_map: &HashMap<u32, Vec<SeqClickTrace>>,
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
                &client_to_hist_map,
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

    utils::write_to_output_file(result_list);
    // Write metrics to final evaluation file 
    utils::write_to_eval_file(config, top_10, top_10_percent);
}

fn eval_step(
    config: &cli::Config,
    client_target: &u32,
    target_idx: &usize,
    client_to_hist_map: &HashMap<u32, Vec<SeqClickTrace>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) -> (u32, u32, bool, bool) {
    let target_click_trace = client_to_hist_map
        .get(client_target)
        .unwrap()
        .get(*target_idx)
        .unwrap();

    let mut tuples: Vec<(OrderedFloat<f64>, u32)> = Vec::with_capacity(client_to_hist_map.len());

    for (client, click_traces) in client_to_hist_map.into_iter() {
        let samples_idx = client_to_sample_idx_map.get(client).unwrap();
        let sampled_click_trace: Vec<SeqClickTrace> = samples_idx
            .into_iter()
            .map(|idx| click_traces.get(*idx).unwrap().clone())
            .collect();

        for sample_click_trace in sampled_click_trace.into_iter() {
            let score = compute_alignment_scores(
                &config.fields,
                &config.strategy,
                &config.scope,
                &config.scoring_matrix,
                &target_click_trace,
                &sample_click_trace,
            );
            tuples.push((OrderedFloat(score), client.clone()));
        }
    }
    tuples.sort_unstable_by_key(|k| Reverse(k.0));
    let cutoff: usize = (0.1 * client_to_hist_map.len() as f64) as usize;
    let is_top_10_percent = utils::is_target_in_top_k(client_target, &tuples[..cutoff]);
    let is_top_10: bool = utils::is_target_in_top_k(client_target, &tuples[..1]);
    (
        client_target.clone(),
        tuples[0].1,
        is_top_10,
        is_top_10_percent,
    )
}

fn compute_alignment_scores(
    fields: &Vec<DataFields>,
    strategy: &str,
    scope: &str,
    scoring_matrix: &[isize],
    target_click_trace: &SeqClickTrace,
    ref_click_trace: &SeqClickTrace,
) -> f64 {
    let mut total_align_score = Vec::<f64>::with_capacity(fields.len());

    for field in fields.into_iter() {
        let score = match field {
            DataFields::Website => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_click_trace.website.clone(),
                ref_click_trace.website.clone(),
            ),
            DataFields::Category => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_click_trace.website.clone(),
                ref_click_trace.website.clone(),
            ),
            DataFields::Code => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_click_trace.website.clone(),
                ref_click_trace.website.clone(),
            ),
            DataFields::Location => compute_similarity_score(
                target_click_trace.location.clone(),
                ref_click_trace.location.clone(),
            ),
            DataFields::Day => compute_similarity_score(
                target_click_trace.day.clone(),
                ref_click_trace.day.clone(),
            ),
            DataFields::Hour => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_click_trace.website.clone(),
                ref_click_trace.website.clone(),
            ),
        };
        total_align_score.push(score)
    }
    // Compute the final score by averaging the indivdual scores
    let avg_score = total_align_score.iter().sum::<f64>() / total_align_score.len() as f64;
    avg_score
}

fn compute_sequence_alignment(
    strategy: &str,
    scope: &str,
    scoring_matrix: &[isize],
    target: Vec<u32>,
    reference: Vec<u32>,
) -> f64 {
    let set: AlignmentSet<InMemoryAlignmentMatrix> = match strategy {
        "nw" => {
            let strategy = NeedlemanWunsch::new(
                scoring_matrix[0],
                scoring_matrix[1],
                scoring_matrix[2],
                scoring_matrix[3],
            );
            AlignmentSet::new(target.len(), reference.len(), strategy, |x, y| {
                target[x] == reference[y]
            })
            .unwrap()
        }
        "sw" => {
            let strategy = SmithWaterman::new(
                scoring_matrix[0],
                scoring_matrix[1],
                scoring_matrix[2],
                scoring_matrix[3],
            );
            AlignmentSet::new(target.len(), reference.len(), strategy, |x, y| {
                target[x] == reference[y]
            })
            .unwrap()
        }
        _ => panic!("Error: unknown strategy name supplied: {}", strategy),
    };

    let score = match scope {
        "global" => set.global_score() as f64,
        "local" => set.local_score() as f64,
        _ => panic!("Error: unknown scope name supplied: {}", scope),
    };
    score
}

fn compute_similarity_score<T: std::cmp::PartialEq>(target: T, reference: T) -> f64 {
    let mut score = 0.0;
    if target == reference {
        score = 1.0;
    } else {
        score = 0.0;
    }
    score
}
