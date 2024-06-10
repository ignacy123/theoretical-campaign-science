use std::net::TcpListener;

use ark_serialize::Write;
use clap::Parser;
use psi::{
    crypto::{
        deserialize_encoded_secrets, deserialize_encoded_shares, evaluate_secret, intersection,
        serialize_secret, serialize_secrets,
    },
    network::read_all,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ServerArgs {
    #[arg(short)]
    server_address: String,
}

fn main() -> std::io::Result<()> {
    let args = ServerArgs::parse();
    let listener = TcpListener::bind(args.server_address).unwrap();

    loop {
        println!("Waiting for Alice and Bob to connect");
        let mut alice_connection = listener.incoming().next().unwrap().unwrap();
        println!("Waiting for Bob");
        let mut bob_connection = listener.incoming().next().unwrap().unwrap();

        println!("Waiting for hashed elements");
        let alice_hashed = deserialize_encoded_secrets(read_all(&alice_connection));
        let bob_hashed = deserialize_encoded_secrets(read_all(&bob_connection));

        let inter = intersection(&alice_hashed, &bob_hashed);
        println!("The intersection is of size {}", inter.len());

        println!("Sending intersection");
        alice_connection.write(&serialize_secrets(&inter)).unwrap();
        alice_connection.shutdown(std::net::Shutdown::Both).unwrap();
        bob_connection.write(&serialize_secrets(&inter)).unwrap();
        bob_connection.shutdown(std::net::Shutdown::Both).unwrap();

        println!("Waiting for Alice and Bob to connect");
        alice_connection = listener.incoming().next().unwrap().unwrap();
        println!("Waiting for Bob");
        bob_connection = listener.incoming().next().unwrap().unwrap();

        println!("Waiting for shares from Alice and Bob");
        let alice_shares = deserialize_encoded_shares(read_all(&alice_connection));
        println!("Waiting for shares from Bob");
        let bob_shares = deserialize_encoded_shares(read_all(&bob_connection));
        let steve_secret = serialize_secret(evaluate_secret(&alice_shares, &bob_shares));

        println!("Sending calculated secret");
        alice_connection.write_all(&steve_secret).unwrap();
        bob_connection.write_all(&steve_secret).unwrap();
    }
}
