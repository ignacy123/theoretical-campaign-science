use std::collections::HashSet;
use ark_bls12_381::Fr;
use ark_poly::{Polynomial};
use ark_std::{One, UniformRand};
use sha256::{digest};


fn main() {
    // setup
    let mut x_rng = rand::thread_rng();
    let a_pk = Fr::rand(&mut x_rng).to_string();
    let b_pk = a_pk.clone();

    // Alice
    let a_data: Vec<&str> = vec!["a", "b", "c", "d", "e"];
    let a_encoded : Vec<String> = a_data.iter().map(|x| {
        let concat = format!("{a_pk}{x}");
        digest(&concat)
    }).collect();

    // Bob
    let b_data: Vec<&str> = vec!["b", "c", "e", "f", "g", "h"];
    let b_encoded : Vec<String> = b_data.iter().map(|x| {
        let concat = format!("{b_pk}{x}");
        digest(&concat)
    }).collect();

    // Steve
    let s_encoded_a = a_encoded.clone();
    let s_encoded_b = b_encoded.clone();
    let s_encoded_a_set : HashSet<String> = s_encoded_a.into_iter().collect();
    let s_encoded_b_set : HashSet<String> = s_encoded_b.into_iter().collect();
    let intersection : HashSet<&String> = s_encoded_a_set.intersection(&s_encoded_b_set).collect();
    let intersection_size = intersection.len();

    // Alice
    let a_result = intersection_size;

    // Bob
    let b_result = intersection_size;

    // println!("{:?}", a_encoded);
    // println!("{:?}", b_encoded);
    // println!("{}", a_result);
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