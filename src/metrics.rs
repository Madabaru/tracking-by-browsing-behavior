use crate::maths;

use nalgebra::EuclideanNorm;
use nalgebra::LpNorm;


use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;
use std::f64::consts::E;


pub enum DistanceMetrics {
    Euclidean,
    Manhatten,
    Cosine,
    Jaccard,
    Bhattacharyya,
    KullbrackLeibler,
    TotalVariation,
    JeffriesMatusita,
    ChiSquared,
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
            "jeffries_matusita" => Ok(Self::JeffriesMatusita),
            "chi_quared" => Ok(Self::ChiSquared),
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

pub fn bhattacharyya_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let dist = target_matrix.zip_fold(&ref_matrix, 0.0, |acc, a, b| {
        let sqrt = (a * b).sqrt();
        acc + sqrt
    });
    -f64::log(dist, E)
}


pub fn kullbrack_leibler_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let eps  = f64::EPSILON;
    let dist = target_matrix.zip_fold(&ref_matrix, 0.0, |acc, a, b| {
        let a = a + eps;
        let b = b + eps;
        let mul = a * f64::log(b / a, E);
        acc + mul
    });
    -dist
}


pub fn total_variation_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let dist = target_matrix.zip_fold(&ref_matrix, 0.0, |acc, a, b| {
        let diff = (a - b).abs();
        f64::max(acc, diff)
    });
    dist
}


pub fn jeffries_matusita_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let sq_sum = target_matrix.zip_fold(&ref_matrix, 0.0, |acc, a, b| {
        let diff = a.sqrt() - b.sqrt();
        let sq = f64::powi(diff, 2);
        acc + sq
    });
    let dist: f64 = sq_sum.sqrt();
    dist
}


pub fn chi_squared_dist(target_vec: Vec<u32>, ref_vec: Vec<u32>) -> f64 {
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let eps  = f64::EPSILON;
    let sum = target_matrix.zip_fold(&ref_matrix, 0.0, |acc, a, b| {
        let a = a + eps;
        let b = b + eps;
        let dff_counter = f64::powi(a - b, 2);
        let diff_denominator = a + b;
        acc + (dff_counter / diff_denominator) 
    });
    let dist: f64 = sum * 0.5;
    dist
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
    let inter = target_set.intersection(&ref_set).count() as f64;
    let union = target_set.union(&ref_set).count() as f64;
    let dist: f64 = inter / union;
    1.0-dist
}


