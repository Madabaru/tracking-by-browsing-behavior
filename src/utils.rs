
use ndarray::{Array};

// Create vector of zeros 
pub fn zeros(size: usize) -> Vec<i32> {
    vec![0; size]
}

// 
pub fn euclidean_dist(target_vec: &Vec<i32>, ref_vec: &Vec<i32>) -> f64 {
    let target_arr = Array::from(target_vec.clone());
    let ref_arr = Array::from(ref_vec.clone());
    let diff = target_arr - ref_arr;
    let pow = diff.mapv(|diff: i32| diff.pow(2));
    let sum: f64= pow.sum().into();
    let sqrt: f64 = sum.sqrt();
    return sqrt;
}