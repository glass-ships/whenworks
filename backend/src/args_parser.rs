use super::*;
use crate::log;
use std::process::exit;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

const HELP_MESSAGE: &str = 
"Usage: whenworks [OPTIONS]

Options:
    -h, --help          Print this help message
    -v, --version       Print the version

    -d, --debug         Print debug messages
    -q, --quiet         Don't print anything to stdout

    -a, --addr          Specify the address to listen on
    -D, --db            Specify the database file to use
    -i, --index         Specify the index file to use";

#[derive(Debug)]
pub struct Args {
    pub addr: SocketAddr,
    pub db_file: &'static str,
    pub index_file: &'static str,
    pub debug: bool,
    pub quiet: bool,
}

pub static mut ARGS: Args = Args {
    // TODO change to &'static str
    addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    db_file: "db.bin",
    index_file: "index.html",
    debug: false,
    quiet: false
};

pub fn init_args() {
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                log!("{HELP_MESSAGE}");
                exit(0);
            },
            "--version" | "-v" => {
                log!("{}", env!("CARGO_PKG_VERSION"));
                exit(0);
            },
            "--addr" | "-a" => {
                let Some(addr) = args.next() else {
                    log!(FATAL, "Missing address");
                    exit(0);
                };
                unsafe { 
                    ARGS.addr = addr.parse::<SocketAddr>().unwrap_or_else(|_| {
                        log!(FATAL, "Invalid address");
                        exit(1);
                    }); 
                }
            },
            "--db" | "-D" => {
                let Some(db_file) = args.next() else {
                    log!(FATAL, "Missing database file");
                    exit(1);
                };
                unsafe { ARGS.db_file = Box::leak(db_file.into_boxed_str()); }
            },
            "--index" | "-i" => {
                let Some(index_file) = args.next() else {
                    log!(FATAL, "Missing index file");
                    exit(1);
                };
                unsafe { ARGS.index_file = Box::leak(index_file.into_boxed_str()); }
            },
            "--debug" | "-d" => { unsafe { ARGS.debug = true; } },
            "--quiet" | "-q" => { unsafe { ARGS.quiet = true; } },
            _ => {
                log!(FATAL, "Invalid argument");
                exit(1);
            },
        }
    }

    log!(DEBUG, "Args: {:?}", unsafe { &ARGS });
}

