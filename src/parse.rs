use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::string::ParseError;

use ini::Properties;

use crate::structs::ClickTrace;
use crate::structs::Record;

#[derive(PartialEq)]
pub enum DataFields {
    Website,
    Code,
    Location,
    Category,
}

impl FromStr for DataFields {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "website" => Ok(Self::Website),
            "code" => Ok(Self::Code),
            "location" => Ok(Self::Location),
            "category" => Ok(Self::Category),
            x => panic!("wrong data field supplied: {:?}", x),
        }
    }
}

pub fn parse_to_histogram(
    conf: &Properties,
) -> Result<HashMap<u32, Vec<ClickTrace>>, Box<dyn Error>> {
    let path = conf.get("path").unwrap();
    let max_click_trace_len = conf
        .get("max_cick_tace_len")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let min_click_trace_len = conf
        .get("min_click_trace_len")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let min_num_click_traces = conf
        .get("min_num_click_traces")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let delay_limit = conf.get("delay_limit").unwrap().parse::<f64>().unwrap();

    let mut prev_time: f64 = 0.0;
    let mut prev_client = String::new();
    let mut click_trace_len: usize = 0;
    let mut client_id: u32 = 0;

    let mut client_to_histogram_map: HashMap<u32, Vec<ClickTrace>> = HashMap::new();
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_client != record.client_id && !prev_client.is_empty() {
            client_id += 1;
        }

        if !client_to_histogram_map.contains_key(&client_id) {
            client_to_histogram_map.insert(client_id, Vec::with_capacity(10));
        }

        let click_traces_list = client_to_histogram_map.get_mut(&client_id).unwrap();

        if click_traces_list.is_empty()
            || click_trace_len >= max_click_trace_len
            || record.timestamp - prev_time >= delay_limit
        {
            if click_trace_len < min_click_trace_len && !click_traces_list.is_empty() {
                click_traces_list.pop();
            }

            let click_trace = ClickTrace {
                website: HashMap::new(),
                code: HashMap::new(),
                location: HashMap::new(),
                category: HashMap::new(),
            };
            click_traces_list.push(click_trace);
            click_trace_len = 0;
        }

        let current_click_trace = click_traces_list.last_mut().unwrap();

        *current_click_trace
            .website
            .entry(record.website.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .code
            .entry(record.code.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .location
            .entry(record.location.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .category
            .entry(record.category.clone())
            .or_insert(0) += 1;

        prev_time = record.timestamp;
        prev_client = record.client_id;
        click_trace_len += 1;
    }

    // Remove any client with less than the minimum number of click traces
    client_to_histogram_map.retain(|_, value| value.len() >= min_num_click_traces);
    Ok(client_to_histogram_map)
}
