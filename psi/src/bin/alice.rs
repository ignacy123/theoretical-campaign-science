use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use clap::Parser;
use psi::{
    crypto::{
        deserialize_encoded_secrets, deserialize_point, deserialize_polynomial, encode_secrets,
        evaluate, evaluate_at_zero, extract_result, random_polynomial, serialize_points,
        serialize_polynomial, serialize_secrets,
    },
    network::{read_all, read_usize},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ClientArgs {
    #[arg(short)]
    server_address: String,
    #[arg(short)]
    client_address: Option<String>,
    #[arg(long, short, action)]
    primary: bool,
}

fn connect_to_other(args: &ClientArgs) -> TcpStream {
    if args.primary {
        println!("I'm primary, awaiting connection from other...");
        let listener = TcpListener::bind(args.client_address.clone().unwrap()).unwrap();
        listener.incoming().next().unwrap().unwrap()
    } else {
        TcpStream::connect(args.client_address.clone().unwrap()).unwrap()
    }
}

fn exchange_lengths(other_connection: &mut TcpStream, my_len: usize) -> usize {
    println!("Sending my length");
    other_connection.write_all(&my_len.to_be_bytes()).unwrap();
    println!("Receiving length from other...");
    read_usize(&other_connection)
}

fn main() -> std::io::Result<()> {
    let args = ClientArgs::parse();

    let secrets = if args.primary {
        vec!["a", "b", "c", "d", "e"]
    } else {
        vec!["b", "c", "e", "f", "g", "h"]
    };
    let mut other_connection = connect_to_other(&args);
    let my_len = secrets.len();
    let other_len = exchange_lengths(&mut other_connection, my_len);
    println!("Other length is {}", other_len);

    let encoded = encode_secrets(&secrets);
    let encoded_serialized = serialize_secrets(&encoded);
    let mut server_connection = TcpStream::connect(&args.server_address).unwrap();
    println!("Sending elements to server");
    server_connection.write(&encoded_serialized)?;
    server_connection
        .shutdown(std::net::Shutdown::Write)
        .unwrap();

    let server_intersection = deserialize_encoded_secrets(read_all(&server_connection));
    println!(
        "Size of server intersection is {}",
        server_intersection.len()
    );

    let random_poly;
    if args.primary {
        random_poly = random_polynomial(my_len + other_len - server_intersection.len());
        other_connection
            .write_all(&serialize_polynomial(&random_poly))
            .unwrap();
        other_connection.shutdown(std::net::Shutdown::Both).unwrap();
    } else {
        random_poly = deserialize_polynomial(read_all(&other_connection));
    }
    let shares = evaluate(&random_poly, &encoded);
    server_connection = TcpStream::connect(args.server_address.clone()).unwrap();
    server_connection
        .write_all(&serialize_points(shares))
        .unwrap();
    server_connection
        .shutdown(std::net::Shutdown::Write)
        .unwrap();
    let server_secret = deserialize_point(read_all(&server_connection));
    let my_secret = evaluate_at_zero(&random_poly);

    if server_secret != my_secret {
        println!("AAA");
    }

    let result = extract_result(secrets, server_intersection);
    println!("I think the intersection is {:?}", result);

    Ok(())
}
