use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use std::string::ParseError;
use std::time::{Duration, UNIX_EPOCH};

use chrono::prelude::DateTime;
use chrono::Datelike;
use chrono::{Timelike, Utc};

use crate::cli::Config;
use crate::maths;
use crate::structs::ClickTrace;
use crate::structs::Record;

#[derive(PartialEq, Debug)]
pub enum DataFields {
    Website,
    Code,
    Location,
    Category,
    Day,
    Hour
}

impl FromStr for DataFields {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "website" => Ok(Self::Website),
            "code" => Ok(Self::Code),
            "location" => Ok(Self::Location),
            "category" => Ok(Self::Category),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            x => panic!("wrong data field supplied: {:?}", x),
        }
    }
}

pub fn parse_to_hist(config: &Config) -> Result<HashMap<u32, Vec<ClickTrace>>, Box<dyn Error>> {
    let mut prev_time: f64 = 0.0;
    let mut prev_client = String::new();
    let mut prev_location = String::new();
    let mut click_trace_len: usize = 0;
    let mut client_id: u32 = 0;

    let mut client_to_hist_map: HashMap<u32, Vec<ClickTrace>> = HashMap::new();
    let mut reader = csv::Reader::from_path(&config.path)?;

    // reader.set_headers(csv::StringRecord::from(vec!["client_id", "timestamp", "website", "code", "location", "category"]));

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_client != record.client_id && !prev_client.is_empty() {
            client_id += 1;
        }

        if !client_to_hist_map.contains_key(&client_id) {
            client_to_hist_map.insert(client_id, Vec::with_capacity(10));
        }

        let click_traces_list = client_to_hist_map.get_mut(&client_id).unwrap();

        if click_traces_list.is_empty()
            || click_trace_len >= config.max_click_trace_len
            || record.timestamp - prev_time >= config.delay_limit
            || prev_location != record.location
        {
            if !click_traces_list.is_empty() {
                if click_trace_len < config.min_click_trace_len
                    || click_traces_list.last().unwrap().click_rate > config.max_click_rate
                    || click_traces_list.last().unwrap().end_time
                        - click_traces_list.last().unwrap().start_time
                        > config.max_click_trace_duration
                    || (prev_location != record.location
                        && record.timestamp - prev_time < config.delay_limit)
                {
                    click_traces_list.pop();
                }
            }

            let click_trace = ClickTrace {
                website: HashMap::new(),
                code: HashMap::new(),
                location: record.location.clone(),
                category: HashMap::new(),
                hour: maths::zeros_u32(24),
                day: maths::zeros_u32(7),
                start_time: record.timestamp,
                end_time: record.timestamp,
                click_rate: 0.0,
            };
            click_traces_list.push(click_trace);
            click_trace_len = 0;
        }

        let current_click_trace = click_traces_list.last_mut().unwrap();

        // Extract day and hour from unix timestamp
        let date = UNIX_EPOCH + Duration::from_secs_f64(record.timestamp.clone());
        let datetime = DateTime::<Utc>::from(date);
        // Convert from u32 to usize
        let hour_index: usize = usize::try_from(datetime.hour()).unwrap();
        let day_index: usize = usize::try_from(datetime.weekday().num_days_from_monday()).unwrap();

        current_click_trace.hour[hour_index] += 1;
        current_click_trace.day[day_index] += 1;
        current_click_trace.end_time = record.timestamp;
        current_click_trace.click_rate = click_trace_len as f64
            / (current_click_trace.end_time - current_click_trace.start_time);

        *current_click_trace
            .website
            .entry(record.website.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .code
            .entry(record.code.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .category
            .entry(record.category.clone())
            .or_insert(0) += 1;

        prev_time = record.timestamp;
        prev_client = record.client_id;
        prev_location = record.location;
        click_trace_len += 1;
    }

    // Remove any client with less than the minimum number of click traces
    println!("{:?}", client_to_hist_map.keys().len());
    client_to_hist_map.retain(|_, value| value.len() >= config.min_num_click_traces);
    println!("{:?}", client_to_hist_map.keys().len());
    Ok(client_to_hist_map)
}
