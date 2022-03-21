use crate::utils;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeqClickTrace {
    pub url: Vec<u32>,
    pub domain: Vec<u32>,
    pub category: Vec<u32>,
    pub hour: Vec<u32>,
    pub day: u32,
    pub start_time: f64,
    pub end_time: f64,
    pub click_rate: f64,
    pub age: String,
    pub gender: String,
}

pub fn gen_typical_click_trace(click_traces: &Vec<SeqClickTrace>) -> SeqClickTrace {
    // Get length of typical click trace by majority vote
    let lengths: Vec<usize> = click_traces.iter().map(|cl| cl.url.len()).collect();
    let typical_length = utils::get_most_freq_element(&lengths);

    // Get typical day
    let days: Vec<u32> = click_traces.iter().map(|cl| cl.day).collect();
    let typical_day = utils::get_most_freq_element(&days);

    // Get typical click rate
    let click_rates: Vec<f64> = click_traces.iter().map(|cl| cl.click_rate).collect();
    let typical_click_rate: f64 = click_rates.iter().sum::<f64>() / click_rates.len() as f64;

    // Get typical age
    let ages: Vec<&str> = click_traces.iter().map(|cl| cl.age.as_str()).collect();
    let typical_age: &str = utils::get_most_freq_element(&ages);

    // Get typical gender
    let genders: Vec<&str> = click_traces.iter().map(|cl| cl.gender.as_str()).collect();
    let typical_gender: &str = utils::get_most_freq_element(&genders);

    // Get typical url
    let mut typical_urls: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_urls.iter_mut().enumerate() {
        let urls: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.url.len() > i)
            .map(|cl| cl.url[i])
            .collect();
        let typical_url = utils::get_most_freq_element(&urls);
        *x = typical_url;
    }

    // Get typical domain
    let mut typical_domains: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_domains.iter_mut().enumerate() {
        let domains: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.domain.len() > i)
            .map(|cl| cl.domain[i])
            .collect();
        let typical_domain = utils::get_most_freq_element(&domains);
        *x = typical_domain;
    }

    // Get typical category
    let mut typical_categories: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_categories.iter_mut().enumerate() {
        let categories: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.category.len() > i)
            .map(|cl| cl.category[i])
            .collect();
        let typical_category = utils::get_most_freq_element(&categories);
        *x = typical_category;
    }

    // Get typical hour
    let mut typical_hours: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_hours.iter_mut().enumerate() {
        let hours: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.hour.len() > i)
            .map(|cl| cl.hour[i])
            .collect();
        let typical_hour = utils::get_most_freq_element(&hours);
        *x = typical_hour;
    }

    // Create typical click trace from typical values
    let typical_click_trace = SeqClickTrace {
        url: typical_urls,
        domain: typical_domains,
        category: typical_categories,
        hour: typical_hours,
        day: typical_day,
        start_time: 0.0,
        end_time: 0.0,
        click_rate: typical_click_rate,
        gender: typical_gender.to_string(),
        age: typical_age.to_string(),
    };
    typical_click_trace
}
