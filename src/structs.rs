use serde::Deserialize;
use ndarray::{Array, ArrayBase, array};

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
    pub website: Vec<i32>,
    pub code: Vec<i32>,
    pub location: Vec<i32>,
    pub category: Vec<i32>,
}


