use crate::cli::Config;
use crate::frequency::{trace::FreqTrace, maths};
use crate::sequence::trace::SeqTrace;

use chrono::{prelude::DateTime, Datelike, Timelike, Utc};
use indexmap::IndexSet;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    error::Error,
    fmt::Display,
    str::FromStr,
    string::ParseError,
    time::{Duration, UNIX_EPOCH},
};

#[derive(Debug, Deserialize)]
pub struct Record {
    pub user_id: String,
    pub timestamp: f64,
    pub url: String,
    pub category: String,
    pub active_seconds: u64,
    pub domain: String,
    pub gender: String,
    pub age: String,
}

#[derive(PartialEq, Debug)]
pub enum DataFields {
    Url,
    Domain,
    Gender,
    Age,
    ActiveSeconds,
    Category,
    Day,
    Hour
}

impl Display for DataFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for DataFields {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "url" => Ok(Self::Url),
            "domain" => Ok(Self::Domain),
            "age" => Ok(Self::Age),
            "category" => Ok(Self::Category),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "gender" => Ok(Self::Gender),
            "active_seconds" => Ok(Self::ActiveSeconds),
            x => panic!("Error: Wrong data field supplied: {:?}", x),
        }
    }
}

/// Parses the raw data into a convenient tree map for the histogram-based approach.
pub fn parse_to_frequency(
    config: &Config,
) -> Result<BTreeMap<u32, Vec<FreqTrace>>, Box<dyn Error>> {
    let mut prev_time: f64 = 0.0;
    let mut prev_user = String::new();
    let mut trace_len: usize = 0;
    let mut user_id: u32 = 0;

    let mut user_to_freq_map: BTreeMap<u32, Vec<FreqTrace>> = BTreeMap::new();
    let mut reader = csv::Reader::from_path(&config.path)?;

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_user != record.user_id && !prev_user.is_empty() {
            // Check last trace added to previous user
            let prev_traces_list = user_to_freq_map.get_mut(&user_id).unwrap();
            if !prev_traces_list.is_empty() {
                if trace_len < config.min_trace_len {
                    prev_traces_list.pop();
                }
            }
            user_id += 1;
        }

        if !user_to_freq_map.contains_key(&user_id) {
            user_to_freq_map.insert(user_id, Vec::with_capacity(10));
        }

        let traces_list = user_to_freq_map.get_mut(&user_id).unwrap();

        if traces_list.is_empty()
            || trace_len >= config.max_trace_len
            || record.timestamp - prev_time >= config.delay_limit
        {
            if !traces_list.is_empty() {
                if trace_len < config.min_trace_len
                    || traces_list.last().unwrap().end_time
                        - traces_list.last().unwrap().start_time
                        > config.max_trace_duration
                {
                    traces_list.pop();
                }
            }

            let trace = FreqTrace {
                url: HashMap::new(),
                domain: HashMap::new(),
                category: HashMap::new(),
                hour: maths::zeros_u32(24),
                day: maths::zeros_u32(7),
                start_time: record.timestamp,
                end_time: record.timestamp,
                age: record.age,
                gender: record.gender,
            };
            traces_list.push(trace);
            trace_len = 0;
        }

        let current_trace = traces_list.last_mut().unwrap();

        // Extract day and hour from unix timestamp
        let date = UNIX_EPOCH + Duration::from_secs_f64(record.timestamp.clone());
        let datetime = DateTime::<Utc>::from(date);

        // Convert from u32 to usize
        let hour_index: usize = usize::try_from(datetime.hour()).unwrap();
        let day_index: usize = usize::try_from(datetime.weekday().num_days_from_monday()).unwrap();

        current_trace.hour[hour_index] += 1;
        current_trace.day[day_index] += 1;
        current_trace.end_time = record.timestamp;

        *current_trace
            .url
            .entry(record.url.clone())
            .or_insert(0) += 1;
        *current_trace
            .domain
            .entry(record.domain.clone())
            .or_insert(0) += 1;
        *current_trace
            .category
            .entry(record.category.clone())
            .or_insert(0) += 1;

        prev_time = record.timestamp;
        prev_user = record.user_id;
        trace_len += 1;
    }

    // Remove any client with less than the minimum number of traces
    log::info!(
        "Numer of clients before filtering: {:?}",
        user_to_freq_map.keys().len()
    );
    user_to_freq_map.retain(|_, value| value.len() >= config.min_num_traces);
    log::info!(
        "Number of clients after filtering: {:?}",
        user_to_freq_map.keys().len()
    );
    let total_num_traces: usize = user_to_freq_map.iter().map(|(_, val)| val.len()).sum();
    log::info!(
        "Total number of mobility traces: {:?}",
        total_num_traces
    );
    Ok(user_to_freq_map)
}


/// Parses the raw data into a convenient tree map for the sequence aligment-based approach.
pub fn parse_to_sequence(
    config: &Config,
) -> Result<BTreeMap<u32, Vec<SeqTrace>>, Box<dyn Error>> {
    let mut prev_time: f64 = 0.0;
    let mut prev_user = String::new();
    let mut trace_len: usize = 0;
    let mut user_id: u32 = 0;

    let mut user_to_seq_map: BTreeMap<u32, Vec<SeqTrace>> = BTreeMap::new();
    let mut reader = csv::Reader::from_path(&config.path)?;

    let mut url_set: IndexSet<String> = IndexSet::new();
    let mut domain_set: IndexSet<String> = IndexSet::new();
    let mut category_set: IndexSet<String> = IndexSet::new();

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_user != record.user_id && !prev_user.is_empty() {
            // Check last mobility trace added to previous user
            let prev_traces_list = user_to_seq_map.get_mut(&user_id).unwrap();
            if !prev_traces_list.is_empty() {
                if trace_len < config.min_trace_len {
                    prev_traces_list.pop();
                }
            }
            user_id += 1;
        }

        if !user_to_seq_map.contains_key(&user_id) {
            user_to_seq_map.insert(user_id, Vec::with_capacity(10));
        }

        let traces_list = user_to_seq_map.get_mut(&user_id).unwrap();

        if traces_list.is_empty()
            || trace_len >= config.max_trace_len
            || record.timestamp - prev_time >= config.delay_limit
        {
            if !traces_list.is_empty() {
                if trace_len < config.min_trace_len
                    || traces_list.last().unwrap().end_time
                        - traces_list.last().unwrap().start_time
                        > config.max_trace_duration
                {
                    traces_list.pop();
                }
            }

            let trace = SeqTrace {
                url: Vec::with_capacity(10),
                domain: Vec::with_capacity(10),
                category: Vec::with_capacity(10),
                hour: Vec::with_capacity(10),
                day: 0,
                start_time: record.timestamp,
                end_time: record.timestamp,
                age: record.age,
                gender: record.gender,
            };
            traces_list.push(trace);
            trace_len = 0;
        }

        let current_trace = traces_list.last_mut().unwrap();

        // Extract day and hour from unix timestamp
        let date = UNIX_EPOCH + Duration::from_secs_f64(record.timestamp.clone());
        let datetime = DateTime::<Utc>::from(date);

        url_set.insert(record.url.clone());
        domain_set.insert(record.domain.clone());
        category_set.insert(record.category.clone());

        current_trace.hour.push(datetime.hour());
        current_trace.day = datetime.weekday().num_days_from_monday();
        current_trace.end_time = record.timestamp;

        current_trace
            .url
            .push(u32::try_from(url_set.get_full(&record.url).unwrap().0).unwrap());
        current_trace
            .domain
            .push(u32::try_from(domain_set.get_full(&record.domain).unwrap().0).unwrap());
        current_trace
            .category
            .push(u32::try_from(category_set.get_full(&record.category).unwrap().0).unwrap());

        prev_time = record.timestamp;
        prev_user = record.user_id;
        trace_len += 1;
    }

    // Remove any client with less than the minimum number of traces
    log::info!(
        "Number of clients before filtering: {:?}",
        user_to_seq_map.keys().len()
    );
    user_to_seq_map.retain(|_, value| value.len() >= config.min_num_traces);
    log::info!(
        "Number of clients after filtering: {:?}",
        user_to_seq_map.keys().len()
    );
    let total_num_traces: usize = user_to_seq_map.iter().map(|(_, val)| val.len()).sum();
    log::info!(
        "Total number of mobility traces: {:?}",
        total_num_traces
    );
    Ok(user_to_seq_map)
}
