use ndarray::ArrayView1;
use ndarray::Array1;


pub fn convert_vector(vec: &[u32]) -> Vec<f64> {
    let norm_vec: Vec<f64> = vec.into_iter().map(|x| f64::from(*x)).collect();
    norm_vec
}

// Create vector of zeros 
pub fn zeros(size: usize) -> Vec<i32> {
    vec![0; size]
}

pub fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

pub fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e/norm);
    x
}