use std::collections::HashSet;
use ark_bls12_381::{Fr, FrConfig};
use ark_ff::{Fp256, MontBackend, PrimeField};
use ark_poly::{Polynomial};
use ark_std::{One, UniformRand};
use sha256::{digest};

type FrElem = Fp256<MontBackend<FrConfig, 4>>;

fn hash(element: &str, key: FrElem) -> Fr {
    let concat = format!("{element}{key}");
    let digest = digest(&concat);
    Fr::from_le_bytes_mod_order(digest.as_bytes())
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
    let b_data: Vec<&str> = vec!["b", "c", "e", "f", "g", "h"];
    let b_encoded: Vec<FrElem> = b_data.iter().map(|x| hash(x, b_k)).collect();

    // Steve
    let s_encoded_a = a_encoded.clone();
    let s_encoded_b = b_encoded.clone();
    let s_encoded_a_set: HashSet<FrElem> = s_encoded_a.into_iter().collect();
    let s_encoded_b_set: HashSet<FrElem> = s_encoded_b.into_iter().collect();
    let intersection: HashSet<&FrElem> = s_encoded_a_set.intersection(&s_encoded_b_set).collect();
    let intersection_size = intersection.len();

    // Alice
    let a_result = intersection_size;

    // Bob
    let b_result = intersection_size;

    println!("{:?}", a_encoded);
    println!("{:?}", b_encoded);
    println!("{}", a_result);
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