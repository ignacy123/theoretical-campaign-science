use std::{collections::HashSet, hash::Hash};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

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

pub fn serialize<T: CanonicalSerialize>(what: &T) -> Vec<u8> {
    let mut s_encoded = vec![];
    what.serialize_compressed(&mut s_encoded).unwrap();
    s_encoded
}

pub fn deserialize<T: CanonicalDeserialize>(bytes: Vec<u8>) -> T {
    T::deserialize_compressed(&*bytes).unwrap()
}
