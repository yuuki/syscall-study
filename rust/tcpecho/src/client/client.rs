extern crate rand;

use std::io;
use std::io::prelude::*;
use std::net::{TcpStream};

pub fn run(host: &str, port: u16) {
    println!("TCP client connecting to `{}:{}`", host, port);
    let mut stream = TcpStream::connect((host, port)).unwrap();

    let mut wbuf = &mut [0; 128];
    let mut rbuf = &mut [0; 128];

    // randomaize
    for i in 0..128 {
        wbuf[i] = rand::random::<u8>();
    }

    let _ = stream.write(wbuf);
    let _ = stream.read(rbuf);

    io::stdout().write_all(rbuf).unwrap();
}
