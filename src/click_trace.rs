use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ClickTrace {
    pub website: HashMap<String, u32>,
    pub code: HashMap<String, u32>,
    pub location: String,
    pub category: HashMap<String, u32>,
    pub hour: Vec<u32>,
    pub day: Vec<u32>,
    pub start_time: f64,
    pub end_time: f64,
    pub click_rate: f64
}

#[derive(Debug, Clone)]
 pub struct VectClickTrace {
     pub website: Vec<u32>,
     pub code: Vec<u32>,
     pub location: Vec<u32>,
     pub category: Vec<u32>,
     pub hour: Vec<u32>,
     pub day: Vec<u32>
 }
