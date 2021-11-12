
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


