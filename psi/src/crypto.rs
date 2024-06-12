use std::collections::HashSet;
use std::env;
use std::ops::{Add, Mul};

use ark_bls12_381::{Fr, FrConfig};
use ark_ff::{Field, Fp256, MontBackend, PrimeField};
use ark_poly::univariate::DensePolynomial;
use ark_poly::DenseUVPolynomial;
use ark_poly::Polynomial;
use ark_std::UniformRand;
use sha256::digest;

pub type FrElem = Fp256<MontBackend<FrConfig, 4>>;
pub type Poly = DensePolynomial<ark_ff::Fp<MontBackend<FrConfig, 4>, 4>>;

fn get_shared_secret() -> FrElem {
    let raw = env::var("SHARED_SECRET").unwrap_or("69".to_string());
    Fr::from(raw.parse::<u64>().unwrap())
}

fn hash(element: &str, key: FrElem) -> FrElem {
    let concat = format!("{element}{key}");
    let digest = digest(&concat);
    Fr::from_le_bytes_mod_order(digest.as_bytes())
}

pub fn extract_result(data: Vec<String>, hashes: Vec<FrElem>) -> Vec<String> {
    let hashes_set: HashSet<FrElem> = hashes.iter().copied().collect();
    let mut result = vec![];
    let key = get_shared_secret();
    for elem in data {
        if hashes_set.contains(&hash(&elem, key)) {
            result.push(elem);
        }
    }
    assert_eq!(result.len(), hashes.len());
    result
}

pub fn encode_data(data: &Vec<String>) -> Vec<FrElem> {
    let seed = get_shared_secret();
    data.iter().map(|x| hash(x, seed)).collect()
}

// polynomials

pub fn random_polynomial(z: usize) -> Poly {
    let mut a_rng = rand::thread_rng();
    let mut a_poly_coeffs: Vec<FrElem> = vec![];
    for _ in 0..z {
        a_poly_coeffs.push(Fr::rand(&mut a_rng))
    }
    DensePolynomial::from_coefficients_vec(a_poly_coeffs)
}

pub fn lagrange_interpolation(points: Vec<(FrElem, FrElem)>) -> DensePolynomial<FrElem> {
    let k = points.len() - 1;
    let mut interpolated_poly = DensePolynomial::from_coefficients_vec(vec![Fr::from(0)]);

    for j in 0..(k + 1) {
        let mut num: DensePolynomial<FrElem> =
            DensePolynomial::from_coefficients_vec(vec![Fr::from(1)]);
        let mut den: FrElem = Fr::from(1);

        for m in 0..(k + 1) {
            if j == m {
                continue;
            }
            num = num.naive_mul(&DensePolynomial::from_coefficients_vec(vec![
                -points[m].0,
                Fr::from(1),
            ]));
            den = den * (points[j].0 - points[m].0);
        }

        let l_j = num.mul(den.inverse().unwrap());
        interpolated_poly = interpolated_poly.add(l_j.mul(points[j].1));
    }

    interpolated_poly
}

pub fn evaluate_at_zero(poly: &Poly) -> FrElem {
    poly.evaluate(&Fr::from(0))
}

pub fn evaluate_at_points(poly: &Poly, points: &Vec<FrElem>) -> Vec<(FrElem, FrElem)> {
    points.iter().map(|x| (*x, poly.evaluate(x))).collect()
}
