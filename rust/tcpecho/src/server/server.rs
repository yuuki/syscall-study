use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!("accept!");
    std::io::copy(&mut stream.try_clone().unwrap(), &mut stream).unwrap();
}

pub fn run(host: &str, port: u16) {
    println!("TCP server binding to `{}:{}`", host, port);
    let listener = TcpListener::bind((host, port)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            },
            Err(e) => { println!("error {}", e); }
        }
    }

    drop(listener);
}
