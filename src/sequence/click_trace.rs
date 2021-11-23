use crate::utils;

#[derive(Debug, Clone)]
pub struct SeqClickTrace {
    pub website: Vec<u32>,
    pub code: Vec<u32>,
    pub location: String,
    pub category: Vec<u32>,
    pub hour: Vec<u32>,
    pub day: u32,
    pub start_time: f64,
    pub end_time: f64,
    pub click_rate: f64,
}

pub fn gen_typical_click_trace(click_traces: &Vec<SeqClickTrace>) -> SeqClickTrace {
    // Get length of typical click trace by majority vote
    let lengths: Vec<usize> = click_traces.iter().map(|cl| cl.website.len()).collect();
    let typical_length = utils::get_most_freq_element(&lengths);

    // Get typical day
    let days: Vec<u32> = click_traces.iter().map(|cl| cl.day).collect();
    let typical_day = utils::get_most_freq_element(&days);

    // Get typical location
    let locations: Vec<&str> = click_traces.iter().map(|cl| cl.location.as_str()).collect();
    let typical_location: &str = utils::get_most_freq_element(&locations);

    // Get typical click rate
    let click_rates: Vec<f64> = click_traces.iter().map(|cl| cl.click_rate).collect();
    let typical_click_rate: f64 = click_rates.iter().sum::<f64>() / click_rates.len() as f64;

    // Get typical website
    let mut typical_websites: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_websites.iter_mut().enumerate() {
        let websites: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.website.len() > i)
            .map(|cl| cl.website[i])
            .collect();
        let typical_website = utils::get_most_freq_element(&websites);
        *x = typical_website;
    }

    // Get typical code
    let mut typical_codes: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_codes.iter_mut().enumerate() {
        let codes: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.code.len() > i)
            .map(|cl| cl.code[i])
            .collect();
        let typical_code = utils::get_most_freq_element(&codes);
        *x = typical_code;
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
        website: typical_websites,
        code: typical_codes,
        location: typical_location.to_string(),
        category: typical_categories,
        hour: typical_hours,
        day: typical_day,
        start_time: 0.0,
        end_time: 0.0,
        click_rate: typical_click_rate,
    };
    typical_click_trace
}
