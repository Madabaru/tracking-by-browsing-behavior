use crate::maths;

use nalgebra::EuclideanNorm;
use nalgebra::LpNorm;

use std::str::FromStr;
use std::string::ParseError;



pub enum DistanceMetrics {
    Euclidean,
    Manhatten,
    // CityBlock,
    // Cosine,
    // NonIntersect,
    // KL,
    // TotalVariation,

}


impl FromStr for DistanceMetrics {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "euclidean" => Ok(Self::Euclidean),
            "manhatten" => Ok(Self::Manhatten),
            x => panic!("Problem opening the file: {:?}", x)
        }
    }
}


pub fn euclidean_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let dist = target_matrix.apply_metric_distance(&ref_matrix, &EuclideanNorm);    
    dist
}

pub fn manhatten_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let dist = target_matrix.apply_metric_distance(&ref_matrix, &LpNorm(1));    
    dist
}