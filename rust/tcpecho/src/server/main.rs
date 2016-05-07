extern crate getopts;
extern crate server;

use getopts::Options;
use std::env;

const NAME: &'static str = "server";
const LOOPBACK_ADDR: &'static str = "127.0.0.1";
const DEFAULT_PORT: u16 = 10100;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("H", "host", "bind hostname or ipaddr", "HOST");
    opts.optopt("P", "port", "bind port", "PORT");
    opts.optflag("h", "help", "display this help and exit");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => panic!("{}", err),
    };

    if matches.opt_present("help") {
        let brief = format!("Usage: {name} [-H HOST] [-P PORT]", name = NAME);
        print!("{}", opts.usage(&brief));
        std::process::exit(-1);
    }
    let host = match matches.opt_str("host") {
        Some(h) => h,
        None    => LOOPBACK_ADDR.to_string(),
    };
    let port = match matches.opt_str("port") {
        Some(p) => match p.trim().parse() {
            Ok(num) => num,
            Err(err) => panic!(err),
        },
        None    => DEFAULT_PORT,
    };

    server::run(host.as_str(), port);
}
