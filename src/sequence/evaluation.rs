use crate::parse::DataFields;
use crate::sequence::click_trace::SeqClickTrace;
use crate::utils;
use crate::{cli, sequence};

use ordered_float::OrderedFloat;
use rayon::prelude::*;
use seal::pair::{AlignmentSet, InMemoryAlignmentMatrix, NeedlemanWunsch, SmithWaterman};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap},
};

pub fn eval(
    config: &cli::Config,
    client_to_seq_map: &BTreeMap<u32, Vec<SeqClickTrace>>,
    client_to_target_idx_map: &HashMap<u32, Vec<usize>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
    client_to_test_idx_map: &HashMap<u32, usize>,
) {
    let nested_result_list: Vec<Vec<(bool, bool, bool)>> = client_to_target_idx_map
        .par_iter()
        .map(|(client, target_idx_list)| {
            eval_step(
                config,
                client,
                &target_idx_list,
                &client_to_seq_map,
                &client_to_sample_idx_map,
                &client_to_test_idx_map,
            )
        })
        .collect();

    let result_list = utils::flatten(nested_result_list);
    let mut top_1_list: Vec<f64> = Vec::with_capacity(result_list.len());
    let mut top_10_list: Vec<f64> = Vec::with_capacity(result_list.len());
    let mut top_10_percent_list: Vec<f64> = Vec::with_capacity(result_list.len());
    for (in_top_1, in_top_10, in_top_10_percent) in result_list.iter() {
        if *in_top_1 {
            top_1_list.push(1.0);
        } else {
            top_1_list.push(0.0);
        }
        if *in_top_10 {
            top_10_list.push(1.0);
        } else {
            top_10_list.push(0.0);
        }
        if *in_top_10_percent {
            top_10_percent_list.push(1.0);
        } else {
            top_10_percent_list.push(0.0);
        }
    }

    let top_1: f64 = utils::mean(&top_1_list);
    log::info!("Rank 1: {:?}", top_1);
    let top_10: f64 = utils::mean(&top_10_list);
    log::info!("Top 10: {:?}", top_10);
    let top_10_percent: f64 = utils::mean(&top_10_percent_list);
    log::info!("Top 10 Percent: {:?}", top_10_percent);

    let top_1_std = utils::std_deviation(&top_1_list);
    let top_10_std = utils::std_deviation(&top_10_list);
    let top_10_percent_std = utils::std_deviation(&top_10_percent_list);

    // Write metrics to final evaluation file
    utils::write_to_file(
        config,
        top_1,
        top_1_std,
        top_10,
        top_10_std,
        top_10_percent,
        top_10_percent_std,
    )
    .expect("Error writing to evaluation file.");
}

pub fn eval_dependent(
    config: &cli::Config,
    client_to_seq_map: &BTreeMap<u32, Vec<SeqClickTrace>>,
    client_to_target_idx_map: &HashMap<u32, Vec<usize>>,
    client_to_sample_idx_map: &mut HashMap<u32, Vec<usize>>
) {
    let nested_result_list: Vec<Vec<(bool, bool, bool)>> = client_to_target_idx_map
        .iter()
        .map(|(client_target, target_idx_list)| {
            eval_step_dependent(
                config,
                client_target,
                &target_idx_list,
                &client_to_seq_map,
                client_to_sample_idx_map
            )
        })
        .collect();

    let result_list = utils::flatten(nested_result_list);
    let mut top_1_list: Vec<f64> = Vec::with_capacity(result_list.len());
    let mut top_10_list: Vec<f64> = Vec::with_capacity(result_list.len());
    let mut top_10_percent_list: Vec<f64> = Vec::with_capacity(result_list.len());
    for (in_top_1, in_top_10, in_top_10_percent) in result_list.iter() {
        if *in_top_1 {
            top_1_list.push(1.0);
        } else {
            top_1_list.push(0.0);
        }
        if *in_top_10 {
            top_10_list.push(1.0);
        } else {
            top_10_list.push(0.0);
        }
        if *in_top_10_percent {
            top_10_percent_list.push(1.0);
        } else {
            top_10_percent_list.push(0.0);
        }
    }

    let top_1: f64 = utils::mean(&top_1_list);
    log::info!("Rank 1: {:?}", top_1);
    let top_10: f64 = utils::mean(&top_10_list);
    log::info!("Top 10: {:?}", top_10);
    let top_10_percent: f64 = utils::mean(&top_10_percent_list);
    log::info!("Top 10 Percent: {:?}", top_10_percent);

    let top_1_std = utils::std_deviation(&top_1_list);
    let top_10_std = utils::std_deviation(&top_10_list);
    let top_10_percent_std = utils::std_deviation(&top_10_percent_list);

    // Write metrics to final evaluation file
    utils::write_to_file(
        config,
        top_1,
        top_1_std,
        top_10,
        top_10_std,
        top_10_percent,
        top_10_percent_std,
    )
    .expect("Error writing to evaluation file.");
}

fn eval_step(
    config: &cli::Config,
    client_target: &u32,
    target_idx_list: &Vec<usize>,
    client_to_seq_map: &BTreeMap<u32, Vec<SeqClickTrace>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
    client_to_test_idx_map: &HashMap<u32, usize>,
) -> Vec<(bool, bool, bool)> {

    let mut result_map: HashMap<u32, OrderedFloat<f64>> = HashMap::new();
    let mut result_tuples_list: Vec<(bool, bool, bool)> = Vec::with_capacity(target_idx_list.len());

    for target_idx in target_idx_list.into_iter() {
        let target_click_trace = client_to_seq_map
            .get(client_target)
            .unwrap()
            .get(*target_idx)
            .unwrap();

        let mut result_tuples: Vec<(u32, OrderedFloat<f64>)> = Vec::with_capacity(client_to_seq_map.len());

        for (client, click_traces) in client_to_seq_map.into_iter() {
            let samples_idx = client_to_sample_idx_map.get(client).unwrap();
            let sampled_click_traces: Vec<SeqClickTrace> = samples_idx
                .into_iter()
                .map(|idx| click_traces.get(*idx).unwrap().clone())
                .collect();

            if config.typical && !config.multiple {
                let typical_click_trace =
                    sequence::click_trace::gen_typical_click_trace(&sampled_click_traces);

                let score = compute_alignment_scores(
                    &config.fields,
                    &config.strategy,
                    &config.scope,
                    &config.scoring_matrix,
                    &target_click_trace,
                    &typical_click_trace,
                );
                result_tuples.push((client.clone(), OrderedFloat(score)));

            } else if !config.typical && !config.multiple {
                for sample_click_trace in sampled_click_traces.into_iter() {
                    let score = compute_alignment_scores(
                        &config.fields,
                        &config.strategy,
                        &config.scope,
                        &config.scoring_matrix,
                        &target_click_trace,
                        &sample_click_trace,
                    );
                    result_tuples.push((client.clone(), OrderedFloat(score)));
                }
            } else {
                let test_idx: usize = client_to_test_idx_map.get(client).unwrap().clone();
                let click_trace: SeqClickTrace = click_traces.get(test_idx).unwrap().clone();
                let score = compute_alignment_scores(
                    &config.fields,
                    &config.strategy,
                    &config.scope,
                    &config.scoring_matrix,
                    &target_click_trace,
                    &click_trace,
                );
                *result_map
                    .entry(client.clone())
                    .or_insert(OrderedFloat(0.0)) += OrderedFloat(score);
            }
        }

        if !config.multiple {
            result_tuples.sort_unstable_by_key(|k| Reverse(k.1));
            let cutoff: usize = (0.1 * client_to_seq_map.len() as f64) as usize;
            let is_top_10_percent = utils::is_target_in_top_k(client_target, &result_tuples[..cutoff]);
            let is_top_10: bool = utils::is_target_in_top_k(client_target, &result_tuples[..10]);
            let is_top_1: bool = client_target.clone() == result_tuples[0].0;
            result_tuples_list.push((is_top_1, is_top_10, is_top_10_percent));
        }
    }

    if config.multiple {
        let mut result_tuples: Vec<(u32, OrderedFloat<f64>)> = result_map.into_iter().collect();
        result_tuples.sort_unstable_by_key(|k| Reverse(k.1));
        let cutoff: usize = (0.1 * client_to_seq_map.len() as f64) as usize;
        let is_top_10_percent = utils::is_target_in_top_k(client_target, &result_tuples[..cutoff]);
        let is_top_10: bool = utils::is_target_in_top_k(client_target, &result_tuples[..10]);
        let is_top_1: bool = client_target.clone() == result_tuples[0].0;
        result_tuples_list.push((is_top_1, is_top_10, is_top_10_percent));
    }
    result_tuples_list
}

fn eval_step_dependent(
    config: &cli::Config,
    client_target: &u32,
    target_idx_list: &Vec<usize>,
    client_to_seq_map: &BTreeMap<u32, Vec<SeqClickTrace>>,
    client_to_sample_idx_map: &mut HashMap<u32, Vec<usize>>,
) -> Vec<(bool, bool, bool)> {

    let mut result_tuples_list: Vec<(bool, bool, bool)> = Vec::with_capacity(target_idx_list.len());

    for target_idx in target_idx_list.into_iter() {
        let target_click_trace = client_to_seq_map
            .get(client_target)
            .unwrap()
            .get(*target_idx)
            .unwrap();

        let mut result_tuples: Vec<(u32, OrderedFloat<f64>)> = Vec::with_capacity(client_to_seq_map.len());

        for (client, click_traces) in client_to_seq_map.into_iter() {
            let samples_idx = client_to_sample_idx_map.get(client).unwrap();
            let sampled_click_traces: Vec<SeqClickTrace> = samples_idx
                .into_iter()
                .map(|idx| click_traces.get(*idx).unwrap().clone())
                .collect();

            for sample_click_trace in sampled_click_traces.into_iter() {
                let score = compute_alignment_scores(
                    &config.fields,
                    &config.strategy,
                    &config.scope,
                    &config.scoring_matrix,
                    &target_click_trace,
                    &sample_click_trace,
                );
                result_tuples.push((client.clone(), OrderedFloat(score)));
            }
        }

        // Decide whether the linkage attack is successful based on simple heuristic
        result_tuples.sort_unstable_by_key(|k| Reverse(k.1));
        let significant = utils::is_significant(&result_tuples);

        if significant {
            let sample_idx_list = client_to_sample_idx_map.get_mut(client_target).unwrap();
            sample_idx_list.push(*target_idx);
        }

        let cutoff: usize = (0.1 * client_to_seq_map.len() as f64) as usize;
        let is_top_10_percent = utils::is_target_in_top_k(client_target, &result_tuples[..cutoff]);
        let is_top_10: bool = utils::is_target_in_top_k(client_target, &result_tuples[..10]);
        let is_top_1: bool = client_target.clone() == result_tuples[0].0;
        result_tuples_list.push((is_top_1, is_top_10, is_top_10_percent));
        
    }
    result_tuples_list
}

fn compute_alignment_scores(
    fields: &Vec<DataFields>,
    strategy: &str,
    scope: &str,
    scoring_matrix: &[isize],
    target_click_trace: &SeqClickTrace,
    sample_click_trace: &SeqClickTrace,
) -> f64 {
    let mut align_scores = Vec::<f64>::with_capacity(fields.len());
    let mut unnormalized_align_scores = Vec::<f64>::with_capacity(fields.len());

    for field in fields.into_iter() {
        let score = match field {
            DataFields::Url => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_click_trace.url.clone(),
                sample_click_trace.url.clone(),
            ),
            DataFields::Category => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_click_trace.category.clone(),
                sample_click_trace.category.clone(),
            ),
            DataFields::Domain => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_click_trace.domain.clone(),
                sample_click_trace.domain.clone(),
            ),
            DataFields::Day => compute_similarity_score(
                target_click_trace.day.clone(),
                sample_click_trace.day.clone(),
            ),
            DataFields::Hour => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_click_trace.hour.clone(),
                sample_click_trace.hour.clone(),
            ),
            DataFields::Gender => compute_similarity_score(
                target_click_trace.gender.clone(),
                sample_click_trace.gender.clone(),
            ),
            DataFields::Age => compute_similarity_score(
                target_click_trace.age.clone(),
                sample_click_trace.age.clone(),
            ),
            DataFields::ClickRate => compute_absolute_error_score(
                target_click_trace.click_rate.clone(),
                sample_click_trace.click_rate.clone(),
            ),
            _ => panic!("Error: unknown field name supplied: {}", field),
        };

        match field {
            DataFields::Url => unnormalized_align_scores.push(score),
            DataFields::Domain => unnormalized_align_scores.push(score),
            DataFields::Category => unnormalized_align_scores.push(score),
            DataFields::Day => align_scores.push(score),
            DataFields::Hour => unnormalized_align_scores.push(score),
            DataFields::Gender => align_scores.push(score),
            DataFields::Age => align_scores.push(score),
            DataFields::ClickRate => align_scores.push(score),
            _ => panic!("Error: unknown field name supplied: {}", field),
        }
    }

    // Normalize scores
    utils::normalize_vector(&mut unnormalized_align_scores);
    align_scores.append(&mut unnormalized_align_scores);

    // Compute the final score by averaging the indivdual scores
    let avg_score = align_scores.iter().sum::<f64>() / align_scores.len() as f64;
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
    let score;
    if target == reference {
        score = 1.0;
    } else {
        score = 0.0;
    }
    score
}

fn compute_absolute_error_score(target: f64, reference: f64) -> f64 {
    let score = f64::abs(target - reference);
    score
}
