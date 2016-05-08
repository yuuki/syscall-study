extern crate rand;

#[macro_use]
extern crate log;

use std::io::prelude::*;
use std::net::{TcpStream};
use std::thread;

fn spawn_with(host: String, port: u16, nmsgs: usize) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut stream = match TcpStream::connect((host.as_str(), port)) {
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

        for _ in 0..nmsgs {
            let _ = stream.write(wbuf);
            let _ = stream.read(rbuf);

            print!(".");
        }
    })
}

pub fn run(host: &str, port: u16, nmsgs: usize, nthreads: usize) {
    println!("--> Connecting to `{}:{}`", host, port);

    let children: Vec<_> = (0..nthreads).map(|_| {
        spawn_with(host.to_string(), port.clone(), nmsgs.clone())
    }).collect();

    for child in children {
        let _ = child.join();
    }

    println!("\n{} threads {} messages sent.", nthreads, nmsgs);
}
