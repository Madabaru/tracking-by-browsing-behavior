use crate::maths;

use nalgebra::EuclideanNorm;
use nalgebra::LpNorm;

use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

pub enum DistanceMetrics {
    Euclidean,
    Manhatten,
    Cosine,
    Jaccard,
    Bhattacharyya,
    KullbrackLeibler,
    TotalVariation,
    // Matusita
}

impl FromStr for DistanceMetrics {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "euclidean" => Ok(Self::Euclidean),
            "manhatten" => Ok(Self::Manhatten),
            "cosine" => Ok(Self::Cosine),
            "jaccard" => Ok(Self::Jaccard),
            "bhattacharyya" => Ok(Self::Bhattacharyya),
            "kullbrack_leibler" => Ok(Self::KullbrackLeibler),
            "total_variation" => Ok(Self::TotalVariation),
            x => panic!("Problem opening the file: {:?}", x),
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

pub fn consine_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let target_matrix = maths::vec_to_matrix(target_vec, false);
    let ref_matrix = maths::vec_to_matrix(ref_vec, false);
    let norms = target_matrix.norm() * ref_matrix.norm();
    if norms > 0. {
        return 1.0 - target_matrix.dot(&ref_matrix) / (target_matrix.norm() * ref_matrix.norm());
    }
    1.0
}

pub fn jaccard_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let non_zero_target: Vec<usize> = target_vec
        .into_iter()
        .enumerate()
        .filter(|&(_, v)| v > 0)
        .map(|(i, _)| i)
        .collect();
    let non_zero_ref: Vec<usize> = ref_vec
        .into_iter()
        .enumerate()
        .filter(|&(_, v)| v > 0)
        .map(|(i, _)| i)
        .collect();
    let target_set: HashSet<usize> = non_zero_target.into_iter().collect();
    let ref_set: HashSet<usize> = non_zero_ref.into_iter().collect();
    let intersec = target_set.intersection(&ref_set).count() as f64;
    let union = target_set.union(&ref_set).count() as f64;
    let dist: f64 = (union - intersec) / union;
    dist
}

pub fn bhattacharyya_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let sq_sum = target_vec
        .into_iter()
        .zip(ref_vec)
        .map(|(a, b)| (a as f64 * b as f64).sqrt())
        .sum();
    -f64::log10(sq_sum)
}

// WIP
pub fn kullbrack_leibler_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let dist = target_vec
        .into_iter()
        .zip(ref_vec)
        .map(|(a, b)| a as f64 * f64::log10(b as f64 / a as f64))
        .sum();
    dist
}

pub fn total_varation_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let dist = target_vec
        .into_iter()
        .zip(ref_vec)
        .map(|(a, b)| (a as i32 - b as i32).abs())
        .max()
        .unwrap();
    dist as f64
}

pub fn matusita_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    0.0
}
