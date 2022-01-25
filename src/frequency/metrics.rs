use crate::frequency::maths;

use num_traits::ToPrimitive;
use nalgebra::EuclideanNorm;
use nalgebra::LpNorm;
use std::{f64::consts::E, str::FromStr};

#[derive(Debug)]
pub enum DistanceMetric {
    Euclidean,
    Manhattan,
    Cosine,
    NonIntersection,
    Bhattacharyya,
    KullbrackLeibler,
    TotalVariation,
    JeffriesMatusita,
    ChiSquared,
}

impl FromStr for DistanceMetric {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<DistanceMetric, Self::Err> {
        match s {
            "euclidean" => Ok(DistanceMetric::Euclidean),
            "manhattan" => Ok(DistanceMetric::Manhattan),
            "cosine" => Ok(DistanceMetric::Cosine),
            "non_intersection" => Ok(DistanceMetric::NonIntersection),
            "bhattacharyya" => Ok(DistanceMetric::Bhattacharyya),
            "kullbrack_leibler" => Ok(DistanceMetric::KullbrackLeibler),
            "total_variation" => Ok(DistanceMetric::TotalVariation),
            "jeffries_matusita" => Ok(DistanceMetric::JeffriesMatusita),
            "chi_quared" => Ok(DistanceMetric::ChiSquared),
            x => panic!("The supplied metric does not exist: {:?}", x),
        }
    }
}

pub fn euclidean_dist<T, U>(target_vec: Vec<T>, ref_vec: Vec<U>) -> f64
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
    U: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
{
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let dist = target_matrix.apply_metric_distance(&ref_matrix, &EuclideanNorm);
    dist
}

pub fn manhattan_dist<T, U>(target_vec: Vec<T>, ref_vec: Vec<U>) -> f64
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
    U: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
{
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let dist = target_matrix.apply_metric_distance(&ref_matrix, &LpNorm(1));
    dist
}

pub fn consine_dist<T, U>(target_vec: Vec<T>, ref_vec: Vec<U>) -> f64
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
    U: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
{
    let target_matrix = maths::vec_to_matrix(target_vec, false);
    let ref_matrix = maths::vec_to_matrix(ref_vec, false);
    let norms = target_matrix.norm() * ref_matrix.norm();
    if norms > 0. {
        return 1.0 - target_matrix.dot(&ref_matrix) / (target_matrix.norm() * ref_matrix.norm());
    }
    1.0
}

pub fn bhattacharyya_dist<T, U>(target_vec: Vec<T>, ref_vec: Vec<U>) -> f64
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
    U: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
{
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let dist = target_matrix.zip_fold(&ref_matrix, 0.0, |acc, a, b| {
        let sqrt = (a * b).sqrt();
        acc + sqrt
    });
    -f64::log(dist, E)
}

pub fn kullbrack_leibler_dist<T, U>(target_vec: Vec<T>, ref_vec: Vec<U>) -> f64
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
    U: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
{
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let eps = f64::EPSILON;
    let dist = target_matrix.zip_fold(&ref_matrix, 0.0, |acc, a, b| {
        let a = a + eps;
        let b = b + eps;
        let mul = a * f64::log(b / a, E);
        acc + mul
    });
    -dist
}

pub fn total_variation_dist<T, U>(target_vec: Vec<T>, ref_vec: Vec<U>) -> f64
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
    U: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
{
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let sum = target_matrix.zip_fold(&ref_matrix, 0.0, |acc, a, b| {
        let diff = (a - b).abs();
        acc + diff
    });
    let dist: f64 = sum * 0.5;
    dist
}

pub fn jeffries_dist<T, U>(target_vec: Vec<T>, ref_vec: Vec<U>) -> f64
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
    U: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
{
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

pub fn chi_squared_dist<T, U>(target_vec: Vec<T>, ref_vec: Vec<U>) -> f64
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
    U: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
{
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let eps = f64::EPSILON;
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

pub fn non_intersection_dist<T, U>(target_vec: Vec<T>, ref_vec: Vec<U>) -> f64
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
    U: Clone + std::cmp::PartialEq + std::fmt::Debug + ToPrimitive,
{
    let n = target_vec.len() as f64;
    let target_matrix = maths::vec_to_matrix(target_vec, true);
    let ref_matrix = maths::vec_to_matrix(ref_vec, true);
    let sum = target_matrix.zip_fold(&ref_matrix, 0.0, |acc, a, b| {    
        let min = f64::min(a, b);
        acc + min
    });
    let dist: f64 = n - sum;
    dist
}
