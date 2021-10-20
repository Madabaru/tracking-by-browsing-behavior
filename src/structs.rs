use serde::Deserialize;

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
    pub website: Vec<u32>,
    pub code: Vec<u32>,
    pub location: Vec<u32>,
    pub category: Vec<u32>,
}


