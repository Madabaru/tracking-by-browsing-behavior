mod cli;
mod frequency;
mod parse;
mod sample;
mod sequence;
mod utils;

use frequency::trace::FreqTrace;
use sequence::trace::SeqTrace;
use simple_logger::SimpleLogger;

use rand::{rngs::StdRng, SeedableRng};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

fn main() {
    
    // Load config
    let config = cli::get_cli_config().unwrap();

    // Set up logger
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_timestamps(true)
        .init()
        .unwrap();

    // Set random seed for reproducability
    let mut rng = StdRng::seed_from_u64(config.seed);

    // Approach 1: Sequence alignment-based
    if config.approach == "sequence" {
        log::info!("Parsing data for sequence alignment-based approach...");
        let user_to_seq_map: BTreeMap<u32, Vec<SeqTrace>> =
            parse::parse_to_sequence(&config).unwrap();

        log::info!("Sampling clients...");
        let user_to_target_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_user_to_target_idx_map(
                &user_to_seq_map,
                &mut rng,
                config.user_sample_size,
                config.target_trace_sample_size,
            );

        log::info!("Sampling click traces per client...");
        let mut user_to_sample_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_user_to_sample_idx_map(
                &user_to_seq_map,
                &mut rng,
                config.trace_sample_size,
            );

        log::info!("Sampling test click traces per client...");
        let user_to_test_idx_map: HashMap<u32, usize> =
            sample::gen_user_to_test_idx_map(&user_to_sample_idx_map, &mut rng);

        if config.dependent {
            log::info!("Starting the evaluation with dependent linkage attacks");
            sequence::evaluation::eval_dependent(
                &config,
                &user_to_seq_map,
                &user_to_target_idx_map,
                &mut user_to_sample_idx_map,
            );
        } else {
            log::info!("Starting the evaluation with independent linkage attacks");
            sequence::evaluation::eval(
                &config,
                &user_to_seq_map,
                &user_to_target_idx_map,
                &user_to_sample_idx_map,
                &user_to_test_idx_map,
            );
        }

    // Approach 2: Histogram-based
    } else {
        log::info!("Parsing data for histogram-based approach...");
        let user_to_freq_map: BTreeMap<u32, Vec<FreqTrace>> =
            parse::parse_to_frequency(&config).unwrap();

        log::info!("Sampling a single target click trace per client...");
        let user_to_target_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_user_to_target_idx_map(
                &user_to_freq_map,
                &mut rng,
                config.user_sample_size,
                config.target_trace_sample_size,
            );

        log::info!("Sampling click traces per client...");
        let mut user_to_sample_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_user_to_sample_idx_map(
                &user_to_freq_map,
                &mut rng,
                config.trace_sample_size,
            );

        log::info!("Sampling test click traces per client...");
        let user_to_test_idx_map: HashMap<u32, usize> =
            sample::gen_user_to_test_idx_map(&user_to_sample_idx_map, &mut rng);

        if config.dependent {
            log::info!("Starting the evaluation with dependent linkage attacks");
            frequency::evaluation::eval_dependent(
                &config,
                &user_to_freq_map,
                &user_to_target_idx_map,
                &mut user_to_sample_idx_map,
            );
        } else {
            log::info!("Starting the evaluation with independent linkage attacks");
            frequency::evaluation::eval(
                &config,
                &user_to_freq_map,
                &user_to_target_idx_map,
                &user_to_sample_idx_map,
                &user_to_test_idx_map,
            );
        }
    }
}
