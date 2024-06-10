use std::collections::HashSet;
use std::env;
use std::hash::Hash;
use std::ops::{Add, Mul};

use ark_bls12_381::{Fr, FrConfig};
use ark_ff::{Field, Fp256, MontBackend, PrimeField};
use ark_poly::univariate::DensePolynomial;
use ark_poly::DenseUVPolynomial;
use ark_poly::Polynomial;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::UniformRand;
use sha256::digest;

type FrElem = Fp256<MontBackend<FrConfig, 4>>;
type Poly = DensePolynomial<ark_ff::Fp<MontBackend<FrConfig, 4>, 4>>;

fn get_shared_secret() -> FrElem {
    let raw = env::var("SHARED_SECRET").unwrap_or("69".to_string());
    Fr::from(raw.parse::<u64>().unwrap())
}

fn hash(element: &str, key: FrElem) -> FrElem {
    let concat = format!("{element}{key}");
    let digest = digest(&concat);
    Fr::from_le_bytes_mod_order(digest.as_bytes())
}

fn lagrange_interpolation(points: Vec<(FrElem, FrElem)>) -> DensePolynomial<FrElem> {
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

pub fn intersection<T: Copy + Eq + Hash>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    let a_set: HashSet<T> = v1.iter().copied().collect();
    let b_set: HashSet<T> = v2.iter().copied().collect();
    a_set.intersection(&b_set).copied().collect()
}

pub fn union<T: Copy + Eq + Hash>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    let a_set: HashSet<T> = v1.iter().copied().collect();
    let b_set: HashSet<T> = v2.iter().copied().collect();
    a_set.union(&b_set).copied().collect()
}

pub fn evaluate_secret(v1: &Vec<(FrElem, FrElem)>, v2: &Vec<(FrElem, FrElem)>) -> FrElem {
    let s_shares_union = union(&v1, &v2);
    let s_interpolated_poly = lagrange_interpolation(s_shares_union);
    s_interpolated_poly.evaluate(&Fr::from(0))
}

pub fn encode_secrets(secrets: &Vec<&str>) -> Vec<FrElem> {
    let seed = get_shared_secret();
    secrets.iter().map(|x| hash(x, seed)).collect()
}

pub fn serialize_secret(secret: FrElem) -> Vec<u8> {
    let mut s_encoded = vec![];
    secret.serialize_compressed(&mut s_encoded).unwrap();
    s_encoded
}

pub fn serialize_secrets(secret: &Vec<FrElem>) -> Vec<u8> {
    let mut s_encoded = vec![];
    secret.serialize_compressed(&mut s_encoded).unwrap();
    s_encoded
}

pub fn serialize_polynomial(poly: &Poly) -> Vec<u8> {
    let mut s_encoded = vec![];
    poly.serialize_compressed(&mut s_encoded).unwrap();
    s_encoded
}

pub fn serialize_points(points: Vec<(FrElem, FrElem)>) -> Vec<u8> {
    let mut s_encoded = vec![];
    points.serialize_compressed(&mut s_encoded).unwrap();
    s_encoded
}

pub fn deserialize_point(bytes: Vec<u8>) -> FrElem {
    FrElem::deserialize_compressed(&*bytes).unwrap()
}

pub fn deserialize_encoded_secrets(bytes: Vec<u8>) -> Vec<FrElem> {
    Vec::deserialize_compressed(&*bytes).unwrap()
}

pub fn deserialize_encoded_shares(bytes: Vec<u8>) -> Vec<(FrElem, FrElem)> {
    Vec::deserialize_compressed(&*bytes).unwrap()
}

pub fn deserialize_polynomial(bytes: Vec<u8>) -> Poly {
    DensePolynomial::deserialize_compressed(&*bytes).unwrap()
}

pub fn random_polynomial(z: usize) -> Poly {
    let mut a_rng = rand::thread_rng();
    let mut a_poly_coeffs: Vec<FrElem> = vec![];
    for _ in 0..z {
        a_poly_coeffs.push(Fr::rand(&mut a_rng))
    }
    DensePolynomial::from_coefficients_vec(a_poly_coeffs)
}

pub fn evaluate(poly: &Poly, points: &Vec<FrElem>) -> Vec<(FrElem, FrElem)> {
    points.iter().map(|x| (*x, poly.evaluate(x))).collect()
}

pub fn evaluate_at_zero(poly: &Poly) -> FrElem {
    poly.evaluate(&Fr::from(0))
}

pub fn extract_result(data: Vec<&str>, hashes: Vec<FrElem>) -> Vec<&str> {
    let hashes_set: HashSet<FrElem> = hashes.iter().copied().collect();
    let mut result = vec![];
    let key = get_shared_secret();
    for elem in data {
        if hashes_set.contains(&hash(elem, key)) {
            result.push(elem);
        }
    }
    assert_eq!(result.len(), hashes.len());
    result
}
