use crate::maths;

use ndarray::Array;

use std::str::FromStr;
use std::string::ParseError;



pub enum DistanceMetrics {
    Euclidean
}


impl FromStr for DistanceMetrics {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "euclidean" => Ok(Self::Euclidean),
            x => panic!("Problem opening the file: {:?}", x)
        }
    }
}


pub fn euclidean_dist(target_vec: &Vec<u32>, ref_vec: &Vec<u32>) -> f64 {
    let target_vec: Vec<f64> = maths::convert_vector(&target_vec);
    let ref_vec: Vec<f64> = maths::convert_vector(&ref_vec);
    let target_arr= Array::from(target_vec);
    let ref_arr = Array::from(ref_vec);
    let target_arr = maths::normalize(target_arr);
    let ref_arr = maths::normalize(ref_arr);
    let diff = target_arr - ref_arr;
    let pow = diff.mapv(|diff: f64| diff.powi(2));
    let sum: f64= pow.sum().into();
    sum.sqrt()
}