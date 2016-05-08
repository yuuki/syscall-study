extern crate rand;

#[macro_use]
extern crate log;

use std::io;
use std::io::prelude::*;
use std::net::{TcpStream};

pub fn run(host: &str, port: u16) {
    println!("--> Connecting to `{}:{}`", host, port);
    let mut stream = match TcpStream::connect((host, port)) {
        Ok(s)  => s,
        Err(err) => {
            error!("{} {host}:{port}", err, host=host, port=port);
            return
        },
    };

    let mut wbuf = &mut [0; 128];
    let mut rbuf = &mut [0; 128];

    // randomaize
    for i in 0..128 {
        wbuf[i] = rand::random::<u8>();
    }

    let _ = stream.write(wbuf);
    let _ = stream.read(rbuf);

    match io::stdout().write_all(rbuf) {
        Ok(_) => {},
        Err(err) => {
            error!("{}", err);
            return
        },
    };
}
