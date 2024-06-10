use std::io::Read;
use std::net::TcpStream;

pub fn read_usize(mut stream: &TcpStream) -> usize {
    let mut buf = vec![0; 8];
    stream.read_exact(&mut buf).unwrap();
    usize::from_be_bytes(buf.try_into().unwrap())
}

pub fn read_all(mut stream: &TcpStream) -> Vec<u8> {
    let mut buf = vec![];
    stream.read_to_end(&mut buf).unwrap();
    buf
}
