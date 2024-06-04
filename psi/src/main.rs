use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, Mul};

use ark_bls12_381::{Fr, FrConfig};
use ark_ff::{Field, Fp256, MontBackend, PrimeField};
use ark_poly::{DenseUVPolynomial, EvaluationDomain, Polynomial};
use ark_poly::univariate::DensePolynomial;
use ark_std::{One, UniformRand};
use ark_std::iterable::Iterable;
use sha256::digest;

type FrElem = Fp256<MontBackend<FrConfig, 4>>;

fn hash(element: &str, key: FrElem) -> Fr {
    let concat = format!("{element}{key}");
    let digest = digest(&concat);
    Fr::from_le_bytes_mod_order(digest.as_bytes())
}

fn intersection<T: Copy + Eq + Hash>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    let a_set: HashSet<T> = v1.iter().copied().collect();
    let b_set: HashSet<T> = v2.iter().copied().collect();
    a_set.intersection(&b_set).copied().collect()
}

fn union<T: Copy + Eq + Hash>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    let a_set: HashSet<T> = v1.iter().copied().collect();
    let b_set: HashSet<T> = v2.iter().copied().collect();
    a_set.union(&b_set).copied().collect()
}

// based on this https://en.wikipedia.org/wiki/Lagrange_polynomial, we were unable to find appropriate function in ark
fn lagrange_interpolation(points: Vec<(FrElem, FrElem)>) -> DensePolynomial<FrElem> {
    let k = points.len() - 1;
    let mut interpolated_poly = DensePolynomial::from_coefficients_vec(vec![Fr::from(0)]);

    for j in 0..(k + 1) {
        let mut num: DensePolynomial<FrElem> = DensePolynomial::from_coefficients_vec(vec![Fr::from(1)]);
        let mut den: FrElem = Fr::from(1);

        for m in 0..(k + 1) {
            if j == m {
                continue;
            }
            num = num.naive_mul(&DensePolynomial::from_coefficients_vec(vec![-points[m].0, Fr::from(1)]));
            den = den * (points[j].0 - points[m].0);
        }

        let l_j = num.mul(den.inverse().unwrap());
        interpolated_poly = interpolated_poly.add(l_j.mul(points[j].1));
    }

    interpolated_poly
}


fn main() {
    // setup
    let mut x_rng = rand::thread_rng();
    let a_k = Fr::rand(&mut x_rng);
    let b_k = a_k.clone();

    // Alice
    let a_data: Vec<&str> = vec!["a", "b", "c", "d", "e"];
    let a_encoded: Vec<FrElem> = a_data.iter().map(|x| hash(x, a_k)).collect();

    // Bob
    let b_data: Vec<&str> = vec!["b", "c", "e", "f", "g", "h", "i"];
    let b_encoded: Vec<FrElem> = b_data.iter().map(|x| hash(x, b_k)).collect();

    // Steve
    let s_encoded_a = a_encoded.clone();
    let s_encoded_b = b_encoded.clone();
    let s_encoded_intersection = intersection(&s_encoded_a, &s_encoded_b);
    let s_z = s_encoded_intersection.len();

    // Alice
    let a_z = s_z;
    let a_n = a_data.len();
    let a_m = b_data.len();     // public knowledge
    let a_zp = a_n + a_m - a_z;
    let mut a_rng = rand::thread_rng();
    let mut a_poly_coeffs: Vec<FrElem> = vec![];
    for _ in 0..a_zp {
        a_poly_coeffs.push(Fr::rand(&mut a_rng))
    }
    let a_poly = DensePolynomial::from_coefficients_vec(a_poly_coeffs);
    let a_shares: Vec<(FrElem, FrElem)> = a_encoded.iter().map(|x| (*x, a_poly.evaluate(x))).collect();

    // Bob
    let b_z = s_z;
    let b_poly = a_poly.clone();
    let b_shares: Vec<(FrElem, FrElem)> = b_encoded.iter().map(|x| (*x, b_poly.evaluate(x))).collect();

    // Steve
    let s_shares_a: Vec<(FrElem, FrElem)> = a_shares.clone();
    let s_shares_b: Vec<(FrElem, FrElem)> = b_shares.clone();
    let s_shares_union = union(&s_shares_a, &s_shares_b);
    let s_interpolated_poly = lagrange_interpolation(s_shares_union);
    let s_poly0 = s_interpolated_poly.evaluate(&Fr::from(0));

    // Alice
    let a_s_poly0 = s_poly0;
    assert_eq!(a_s_poly0, a_poly.evaluate(&Fr::from(0)));

    // Bob
    let b_s_poly0 = s_poly0;
    assert_eq!(b_s_poly0, b_poly.evaluate(&Fr::from(0)));
}


#[cfg(test)]
mod tests {
    use crate::Fr;

    #[test]
    fn it_works() {
        let a = Fr::from(2);
        let b = Fr::from(3);
        let c = Fr::from(5);
        assert_eq!(c, a+b);
    }
}