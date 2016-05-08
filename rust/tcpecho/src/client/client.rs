extern crate rand;

#[macro_use]
extern crate log;

use std::io::prelude::*;
use std::net::{TcpStream};

pub fn run(host: &str, port: u16, msg_num: usize) {
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

    for _ in 0..msg_num {
        let _ = stream.write(wbuf);
        let _ = stream.read(rbuf);

        print!(".");
    }

    println!("\n1 connection {} messages sent.", msg_num);
}
