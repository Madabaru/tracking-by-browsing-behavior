use std::str::FromStr;
use crate::parse::DataFields;


#[derive(Debug)]
pub struct Config {
    pub delay_limit: f64,
    pub fields: Vec<DataFields>,
    pub max_click_trace_len: usize,
    pub min_click_trace_len: usize,
    pub max_click_trace_duration: f64,
    pub max_click_rate: f64,
    pub min_num_click_traces: usize,
    pub client_sample_size: usize,
    pub click_trace_sample_size: usize,
    pub metric: String,
    pub path: String,
    pub seed: u64,
    pub typical: bool
}


pub fn get_cli_config() -> Result<Config, core::fmt::Error>{

    let matches = clap::App::new("Tracking-Users-by-Browsing-Behavior")
        .version("1.0")
        .author("Felix John")
        .arg(
            clap::Arg::new("delay_limit")
                .long("delay_limit")
                .about("Maximum delay between two consecutive clicks.")
                .default_value("1800.0"),
        )
        .arg(
            clap::Arg::new("fields")
                .long("fields")
                .possible_values(&["website", "category", "code", "location", "hour", "day"])
                .about("Data fields to consider for the analysis.")
                .default_values(&["website", "category", "code", "location"])
        )
        .arg(
            clap::Arg::new("max_click_trace_len")
                .long("max_click_trace_len")
                .default_value("1000")
                .about("Maximum length of a single click trace."),
        )
        .arg(
            clap::Arg::new("min_click_trace_len")
                .long("min_click_trace_len")
                .default_value("3")
                .about("Minimum length of a single click trace."),
        )
        .arg(
            clap::Arg::new("max_click_trace_duration")
                .long("max_click_trace_duration")
                .default_value("86400.0")
                .about("Maximum duration of a single click trace."),
        )
        .arg(
            clap::Arg::new("min_num_click_traces")
                .long("min_num_click_traces")
                .default_value("2")
                .about("Minimum number of click traces per client."),
        )
        .arg(
            clap::Arg::new("max_click_rate")
                .long("max_click_rate")
                .default_value("2.0")
                .about("Maximum allowed click rate (number of clicks / time)."),
        )
        .arg(
            clap::Arg::new("client_sample_size")
                .long("client_sample_size")
                .default_value("10")
                .about("Number of clients to sample."),
        )
        .arg(
            clap::Arg::new("click_trace_sample_size")
                .long("click_trace_sample_size")
                .default_value("3")
                .about("Number of click traces to sample per client"),
        )
        .arg(
            clap::Arg::new("metric")
                .long("metric")
                .default_value("euclidean")
                .about("Distance metric to compare a pair of click traces.")
                .possible_values(&["euclidean", "manhatten", "cosine", "jaccard", "bhattacharyya", "kullbrack_leibler", "total_variation", "jeffries_matusita", "chi_quared"]),
        )
        .arg(
            clap::Arg::new("path")
                .long("path")
                .default_value("data/test.csv")
                .about("Path to the dataset.")
        )
        .arg(
            clap::Arg::new("seed")
                .long("seed")
                .default_value("0")
                .about("Random seed for reproducability.")
        )
        .arg(
            clap::Arg::new("typical")
                .long("typical")
                .default_value("false")
                .about("Set to true if you want to compute a typical click trace (session) per client.")
        )
        .get_matches();


    let config = Config {
        delay_limit: matches
            .value_of("delay_limit")
            .unwrap_or_default()
            .parse::<f64>().unwrap(),
        metric: matches
            .value_of("metric")
            .map(String::from)
            .unwrap_or_default(),
        max_click_trace_len: matches
            .value_of("max_click_trace_len")
            .unwrap_or_default()
            .parse::<usize>().unwrap(),
        fields: matches
            .values_of_lossy("fields")
            .unwrap_or_default()
            .iter()
            .map(|x| DataFields::from_str(x).unwrap())
            .collect(),
        click_trace_sample_size: matches
            .value_of("click_trace_sample_size")
            .unwrap_or_default()
            .parse::<usize>().unwrap(),
        client_sample_size: matches
            .value_of("client_sample_size")
            .unwrap_or_default()
            .parse::<usize>().unwrap(),
        max_click_rate: matches
            .value_of("max_click_rate")
            .unwrap_or_default()
            .parse::<f64>().unwrap(),
        max_click_trace_duration: matches
            .value_of("max_click_trace_duration")
            .unwrap_or_default()
            .parse::<f64>().unwrap(),
        min_click_trace_len: matches
            .value_of("min_click_trace_len")
            .unwrap_or_default()
            .parse::<usize>().unwrap(),
        min_num_click_traces: matches
            .value_of("min_num_click_traces")
            .unwrap_or_default()
            .parse::<usize>().unwrap(),
        path: matches
            .value_of("path")
            .map(String::from)
            .unwrap_or_default(),
        seed: matches
            .value_of("seed")
            .unwrap_or_default()
            .parse::<u64>().unwrap(),
        typical: matches
            .value_of("typical")
            .unwrap_or_default()
            .parse::<bool>().unwrap(),
    };

    println!("{:?}", config);
    Ok(config)
}
