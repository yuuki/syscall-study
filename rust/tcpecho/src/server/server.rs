#[macro_use]
extern crate log;

use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    std::io::copy(&mut stream.try_clone().unwrap(), &mut stream).unwrap();
}

pub fn run(host: &str, port: u16) {
    println!("--> binding to `{}:{}`", host, port);

    let listener = TcpListener::bind((host, port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                print!(".");
                handle_client(stream);
            },
            Err(err) => { error!("{}", err); }
        }
    }

    drop(listener);
}
