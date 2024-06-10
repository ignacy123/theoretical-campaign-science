use std::io::Read;
use std::net::{TcpListener, TcpStream};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Write};

use crate::crypto::{deserialize, serialize};

pub fn next_connection(listener: &TcpListener) -> TcpStream {
    listener.incoming().next().unwrap().unwrap()
}

pub fn send<T: CanonicalSerialize>(what: &T, mut to: &TcpStream) {
    to.write_all(&serialize(what)).unwrap();
    to.shutdown(std::net::Shutdown::Write).unwrap();
}

pub fn receive<T: CanonicalDeserialize>(mut from: &TcpStream) -> T {
    let mut buf = vec![];
    from.read_to_end(&mut buf).unwrap();
    deserialize(buf)
}
