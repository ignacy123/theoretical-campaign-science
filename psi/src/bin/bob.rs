use ark_bls12_381::{Fr, FrConfig};
use ark_ff::{BigInt, Fp256, MontBackend, PrimeField};
use ark_poly::univariate::DensePolynomial;
use ark_serialize::CanonicalSerialize;
use ark_std::UniformRand;
use sha256::digest;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

type FrElem = Fp256<MontBackend<FrConfig, 4>>;

fn hash(element: &str, key: FrElem) -> Fr {
    let concat = format!("{element}{key}");
    let digest = digest(&concat);
    Fr::from_le_bytes_mod_order(digest.as_bytes())
}

fn main() -> std::io::Result<()> {
    let mut stream_steve = TcpStream::connect("localhost:5555")?;
    let mut stream_bob = TcpListener::bind("localhost:5556");

    // shared
    let mut a_rng = rand::thread_rng();
    let mut a_poly_coeffs: Vec<FrElem> = vec![];
    for _ in 0..a_zp {
        a_poly_coeffs.push(Fr::rand(&mut a_rng))
    }
    let a_poly = DensePolynomial::from_coefficients_vec(a_poly_coeffs);

    // Alice
    let a_k = Fr::from(BigInt!(
        "28687541558529233869482842528302916250861578497012699114277431759472642645619"
    ));
    let a_data: Vec<&str> = vec!["a", "b", "c", "d", "e"];
    let a_encoded: Vec<FrElem> = a_data.iter().map(|x| hash(x, a_k)).collect();
    let mut a_encoded_serialized = vec![];
    a_encoded
        .serialize_compressed(&mut a_encoded_serialized)
        .unwrap();

    stream_steve.write(&a_encoded_serialized)?;
    Ok(())
} // the stream is closed here
