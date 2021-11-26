use nalgebra::MatrixXx1;
use num_traits::ToPrimitive;

// Create vector of u32 zeros
pub fn zeros_u32(size: usize) -> Vec<u32> {
    vec![0; size]
}

// Create vector of f64 zeros
pub fn zeros_f64(size: usize) -> Vec<f64> {
    vec![0.0; size]
}

// Add two vectors element-wise together
pub fn add(vector: Vec<f64>, ref_vector: &Vec<u32>) -> Vec<f64> {
    vector
        .iter()
        .zip(ref_vector)
        .map(|(a, b)| *a + *b as f64)
        .collect()
}

// Transform a vector a nalgebra::matrix
pub fn vec_to_matrix<T>(vector: Vec<T>, norm: bool) -> MatrixXx1<f64>
where
    T: ToPrimitive,
{
    let vector: Vec<f64> = vector.iter().map(|x| x.to_f64().unwrap()).collect();
    let matrix = MatrixXx1::from_vec(vector);
    if norm {
        let matrix = matrix.normalize();
        return matrix;
    }
    matrix
}
