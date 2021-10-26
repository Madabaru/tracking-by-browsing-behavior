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

#[derive(Debug, Clone)]
pub struct ClickTrace {
    pub website: HashMap<String, u32>,
    pub code: HashMap<String, u32>,
    pub location: HashMap<String, u32>,
    pub category: HashMap<String, u32>,
}

#[derive(Debug)]
 pub struct ClickTraceVectorized {
     pub website: Vec<u32>,
     pub code: Vec<u32>,
     pub location: Vec<u32>,
     pub category: Vec<u32>
 }


