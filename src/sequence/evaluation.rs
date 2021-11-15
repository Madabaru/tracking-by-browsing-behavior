use crate::cli;
use crate::parse::DataFields;
use crate::utils;
use crate::sequence::click_trace::SeqClickTrace;

use std::{
    collections::HashMap,
    io::{prelude::*, BufWriter},
};

use seal::pair::{AlignmentSet, InMemoryAlignmentMatrix};

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
            eval_step::<dyn seal::pair::Strategy>(
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
    println!("Rank 1: {:?}", accuracy);
    let top_10: f64 = top_10_count as f64 / result_list.len() as f64;
    println!("Top 10: {:?}", top_10);
    let top_10_percent: f64 = top_10_percent_count as f64 / result_list.len() as f64;
    println!("Top 10 Percent: {:?}", top_10_percent);

    let file = std::fs::File::create("tmp/output").unwrap();
    let mut writer = BufWriter::new(&file);
    for i in result_list {
        write!(writer, "{},{} \n", i.0, i.1).expect("Unable to write to output file.");
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("tmp/evaluation")
        .unwrap();
    write!(
        file,
        "--------------\nExperiment: {:?}\nTop 10: {}\nTop 10 Percent: {}\n",
        config, top_10, top_10_percent
    )
    .expect("Unable to write to evaluation file.")
}

fn eval_step<T: seal::pair::Strategy>(
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

    let strategy: T = utils::get_strategy(config);
    let mut tuples: Vec<(OrderedFloat<f64>, u32)> = Vec::with_capacity(client_to_hist_map.len());

    for (client, click_traces) in client_to_hist_map.into_iter() {
        let samples_idx = client_to_sample_idx_map.get(client).unwrap();
        let sampled_click_trace: Vec<SeqClickTrace> = samples_idx
            .into_iter()
            .map(|idx| click_traces.get(*idx).unwrap().clone())
            .collect();

        for sample_click_trace in sampled_click_trace.into_iter() {
            let score = compute_alignment_scores(&config.fields, &target_click_trace, &sample_click_trace, strategy, &config.scope);
            tuples.push((OrderedFloat(score), client.clone()));
        }
    }
    tuples.sort_unstable_by_key(|k| k.0);
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


fn compute_alignment_scores<T: seal::pair::Strategy>(fields: &Vec<DataFields>, target_click_trace: &SeqClickTrace, ref_click_trace: &SeqClickTrace, strategy: T, scope: &str) -> f64 {

    let mut total_align_score = Vec::<f64>::with_capacity(fields.len());

    for field in fields.into_iter() {
        let score = match field {
            DataFields::Website => compute_sequence_alignment(strategy, scope, target_click_trace.website.clone(), ref_click_trace.website.clone()),
            DataFields::Category => compute_sequence_alignment(strategy, scope, target_click_trace.category.clone(), ref_click_trace.category.clone()),
            DataFields::Code => compute_sequence_alignment(strategy, scope, target_click_trace.code.clone(), ref_click_trace.code.clone()),
            DataFields::Location => compute_similarity_score(target_click_trace.location.clone(), ref_click_trace.location.clone()),
            DataFields::Day => compute_similarity_score(target_click_trace.day.clone(), ref_click_trace.day.clone()),
            DataFields::Hour => compute_sequence_alignment(strategy, scope, target_click_trace.hour.clone(), ref_click_trace.hour.clone()),
        };
        total_align_score.push(score)
    }
    // Compute the final score by averaging the indivdual scores
    let avg_score = total_align_score.iter().sum::<f64>() / total_align_score.len() as f64;
    avg_score
}

fn compute_sequence_alignment<T: seal::pair::Strategy>(strategy: T, scope: &str, target: Vec<u32> , reference: Vec<u32>) -> f64{

    let set: AlignmentSet<InMemoryAlignmentMatrix> = AlignmentSet::new(target.len(), reference.len(), strategy, |x, y| {
        target[x] == reference[y]
    })
    .unwrap();

    let score;
    if scope == "global" {
        score == set.global_score() as f64;
    } else {
        score == set.local_score() as f64;
    }
    score
}

fn compute_similarity_score<T: std::cmp::PartialEq>(target: T, reference: T) -> f64 {
    if target == reference {
        return 1.0
    } else {
        return 0.0
    }
}

