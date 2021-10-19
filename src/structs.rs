use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub client_id: String,
    pub timestamp: f64,
    pub website: String,
    pub code: String,
    pub location: String,
    pub category: String,
}

#[derive(Debug)]
pub struct ClickTrace {
    pub website: HashMap<String, u64>,
    pub code: HashMap<String, u64>,
    pub location: HashMap<String, u64>,
    pub category: HashMap<String, u64>,
}

#[derive(Debug)]
pub struct ClickTraceVectorized {
    pub website: Vec<u64>,
    pub code: Vec<u64>,
    pub location: Vec<u64>,
    pub category: Vec<u64>,
}


