use nalgebra::MatrixXx1;

// Create vector of zeros
pub fn zeros(size: usize) -> Vec<i32> {
    vec![0; size]
}

pub fn vec_to_matrix(vec: Vec<u32>, norm: bool) -> MatrixXx1<f64> {
    let matrix = MatrixXx1::from_vec(vec);
    let matrix = matrix.cast::<f64>();
    if norm {
        let matrix = matrix.normalize();
        return matrix;
    }
    matrix
}
