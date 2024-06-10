use std::net::{TcpListener, TcpStream};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use clap::Parser;
use psi::{
    crypto::{evaluate_secret, intersection, FrElem},
    network::{next_connection, receive, send},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ServerArgs {
    #[arg(short)]
    server_address: String,
}

struct Server {
    listener: TcpListener,
    alice_connection: TcpStream,
    bob_connection: TcpStream,
}

impl Server {
    pub fn new(address: String) -> Server {
        let listener = TcpListener::bind(address).unwrap();
        let alice_connection = next_connection(&listener);
        let bob_connection = next_connection(&listener);
        Server {
            listener,
            alice_connection,
            bob_connection,
        }
    }

    pub fn allow_clients(&mut self) {
        self.alice_connection = next_connection(&self.listener);
        self.bob_connection = next_connection(&self.listener);
    }

    pub fn read_from_clients<T: CanonicalDeserialize>(&self) -> (T, T) {
        (
            receive(&self.alice_connection),
            receive(&self.bob_connection),
        )
    }

    pub fn send_to_clients<T: CanonicalSerialize>(&mut self, what: &T) {
        send(what, &self.alice_connection);
        send(what, &self.bob_connection);
    }
}

fn main() -> std::io::Result<()> {
    let args = ServerArgs::parse();

    loop {
        println!("Awaiting clients");
        let mut server = Server::new(args.server_address.clone());

        println!("Waiting for hashed elements");
        let (alice_hashed, bob_hashed) = server.read_from_clients();

        println!("Sending intersection");
        let inter: Vec<FrElem> = intersection(&alice_hashed, &bob_hashed);
        server.send_to_clients(&inter);

        println!("Waiting for shares from Alice and Bob");
        server.allow_clients();
        let (alice_shares, bob_shares) = server.read_from_clients();
        let steve_secret = evaluate_secret(&alice_shares, &bob_shares);

        println!("Sending calculated secret");
        server.send_to_clients(&steve_secret);
    }
}
