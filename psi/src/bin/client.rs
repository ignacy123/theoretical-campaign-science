use core::panic;
use std::net::{TcpListener, TcpStream};

use clap::Parser;
use psi::{
    crypto::{
        encode_secrets, evaluate, evaluate_at_zero, extract_result, random_polynomial, FrElem, Poly,
    },
    network::{next_connection, receive, send},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ClientArgs {
    #[arg(short)]
    server_address: String,
    #[arg(short)]
    client_address: String,
    #[arg(long, short, action)]
    primary: bool,
}

enum ClientType {
    PrimaryClient(TcpListener),
    SecondaryClient(String),
}

struct Client {
    other_connection: TcpStream,
    server_connection: TcpStream,
    server_address: String,
    kind: ClientType,
    data: Vec<String>,
}

impl Client {
    fn new_primary(client_address: String, server_address: String, data: Vec<String>) -> Client {
        let listener = TcpListener::bind(client_address).unwrap();
        println!("I'm primary, awaiting connection from other...");
        let other_connection = next_connection(&listener);
        Client {
            other_connection,
            server_connection: TcpStream::connect(&server_address).unwrap(),
            server_address,
            kind: ClientType::PrimaryClient(listener),
            data,
        }
    }

    fn new_secondary(client_address: String, server_address: String, data: Vec<String>) -> Client {
        Client {
            other_connection: TcpStream::connect(&client_address).unwrap(),
            server_connection: TcpStream::connect(&server_address).unwrap(),
            server_address,
            kind: ClientType::SecondaryClient(client_address),
            data,
        }
    }

    fn reconnect_to_other(&mut self) {
        match &self.kind {
            ClientType::PrimaryClient(listener) => {
                self.other_connection = next_connection(&listener);
            }
            ClientType::SecondaryClient(client_address) => {
                self.other_connection = TcpStream::connect(client_address).unwrap();
            }
        }
    }

    fn reconnect_to_server(&mut self) {
        self.server_connection = TcpStream::connect(&self.server_address).unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let args = ClientArgs::parse();

    // TODO read actual data from db
    let mut client = if args.primary {
        Client::new_primary(
            args.client_address,
            args.server_address,
            vec![
                "a".to_string(),
                "b".into(),
                "c".into(),
                "d".into(),
                "e".into(),
            ],
        )
    } else {
        Client::new_secondary(
            args.client_address,
            args.server_address,
            vec![
                "b".to_string(),
                "d".into(),
                "e".into(),
                "f".into(),
                "g".into(),
                "h".into(),
            ],
        )
    };

    println!("Exchanging lengths");
    send(&client.data.len(), &client.other_connection);
    let other_len: usize = receive(&client.other_connection);

    println!("Sending elements to server");
    let encoded = encode_secrets(&client.data);
    send(&encoded, &client.server_connection);

    let server_intersection: Vec<FrElem> = receive(&client.server_connection);
    println!("Received intersection");

    client.reconnect_to_other();
    send(&server_intersection, &client.other_connection);
    let other_intersection: Vec<FrElem> = receive(&client.other_connection);
    if server_intersection == other_intersection {
        println!("Validated my intersection against other's");
    } else {
        panic!("SERVER IS A LIAR");
    }

    let random_poly: Poly;
    client.reconnect_to_other();
    match client.kind {
        ClientType::PrimaryClient(_) => {
            random_poly =
                random_polynomial(client.data.len() + other_len - server_intersection.len());
            send(&random_poly, &client.other_connection);
        }
        ClientType::SecondaryClient(_) => {
            random_poly = receive(&client.other_connection);
        }
    }
    let shares = evaluate(&random_poly, &encoded);
    client.reconnect_to_server();
    send(&shares, &client.server_connection);
    println!("Sending shares to server");

    let server_secret: FrElem = receive(&client.server_connection);
    let my_secret = evaluate_at_zero(&random_poly);

    if server_secret == my_secret {
        println!("Server sent back a correct secret");
    } else {
        panic!("SERVER IS A LIAR");
    }

    let result = extract_result(client.data, server_intersection);
    println!("The intersection is {:?}", result);

    Ok(())
}
