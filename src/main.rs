mod cli;
mod frequency;
mod parse;
mod sample;
mod sequence;
mod utils;

use frequency::click_trace::FreqClickTrace;
use sequence::click_trace::SeqClickTrace;

use rand::{rngs::StdRng, SeedableRng};
use std::collections::{HashMap, BTreeMap};

fn main() {

    // Load config
    let config = cli::get_cli_config().unwrap();

    // Set up logger
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // Set random seed for reproducability
    let mut rng = StdRng::seed_from_u64(config.seed);

    // Approach 1: Sequence alignment-based
    if config.approach == "sequence" {

        log::info!("Parsing data for sequence alignment-based approach...");
        let client_to_seq_map: BTreeMap<u32, Vec<SeqClickTrace>> =
            parse::parse_to_sequence(&config).unwrap();

        log::info!("Sampling clients...");
        let client_to_target_idx_map: HashMap<u32, usize> =
            sample::gen_test_data(&client_to_seq_map, &mut rng, config.client_sample_size);

        if !config.typical {

            log::info!("Sampling click traces per client...");
            let client_to_sample_idx_map: HashMap<u32, Vec<usize>> =
                sample::get_train_data(&client_to_seq_map, &mut rng, config.click_trace_sample_size);

            log::info!("Starting the evaluation...");
            sequence::evaluation::eval(
                &config,
                &client_to_seq_map,
                &client_to_target_idx_map,
                &client_to_sample_idx_map,
            );
            
        } else {

            log::info!("Collect all click traces per client...");
            let client_to_sample_idx_map: HashMap<u32, Vec<usize>> =
                sample::get_train_data(&client_to_seq_map, &mut rng, 0);

            log::info!("Starting the evaluation...");
            sequence::evaluation::eval(
                &config,
                &client_to_seq_map,
                &client_to_target_idx_map,
                &client_to_sample_idx_map,
            );
        }
        
    // Approach 2: Frequency-based
    } else {
        
        log::info!("Parsing data for frequency-based approach...");
        let client_to_freq_map: BTreeMap<u32, Vec<FreqClickTrace>> =
            parse::parse_to_frequency(&config).unwrap();

        log::info!("Sampling clients...");
        let client_to_target_idx_map: HashMap<u32, usize> =
            sample::gen_test_data(&client_to_freq_map, &mut rng, config.client_sample_size);

        if !config.typical {

            log::info!("Sampling click traces per client...");
            let client_to_sample_idx_map: HashMap<u32, Vec<usize>> = sample::get_train_data(
                &client_to_freq_map,
                &mut rng,
                config.click_trace_sample_size,
            );

            log::info!("Starting the evaluation...");
            frequency::evaluation::eval(
                &config,
                &client_to_freq_map,
                &client_to_target_idx_map,
                &client_to_sample_idx_map,
            );
        } else {

            log::info!("Collect all click traces per client...");
            let client_to_sample_idx_map: HashMap<u32, Vec<usize>> =
                sample::get_train_data(&client_to_freq_map, &mut rng, 0);

            frequency::evaluation::eval(
                &config,
                &client_to_freq_map,
                &client_to_target_idx_map,
                &client_to_sample_idx_map,
            );
        }
    }
}
