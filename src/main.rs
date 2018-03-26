#[macro_use]
extern crate structopt;

use std::net::{IpAddr, SocketAddr, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::io::{self, Write};

use structopt::StructOpt;

const MAX: u16 = 65535;

#[derive(StructOpt, Debug)]
#[structopt(name = "port_sniffer", about = "A basic port snipper")]
struct Config {
    #[structopt(short = "t", long = "threads", default_value = "100", help = "Number of threads")]
    threads: u16,

    #[structopt(help = "Ip Address", parse(try_from_str))] ip_addr: IpAddr,
}

fn scan(ip: IpAddr, start_port: u16, step: u16, sender: Sender<u16>) {
    let mut port = start_port + 1;
    loop {
        let socket_addr = SocketAddr::new(ip, port as u16);
        match TcpStream::connect(socket_addr) {
            Ok(_) => {
                print!(".");
                let _ = io::stdout().flush();
                sender.send(port).unwrap();
            }
            _ => (),
        }
        if (MAX - port) <= step {
            return;
        }
        port = port + step;
    }
}

fn run(config: Config) {
    let (tx, rx): (Sender<u16>, Receiver<u16>) = mpsc::channel();
    let threads = config.threads;
    let ip_addr = config.ip_addr;

    for p in 0..threads {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            scan(ip_addr, p, threads, thread_tx);
        });
    }

    let mut ports: Vec<u16> = (0..threads).filter_map(|_| rx.recv().ok()).collect();
    ports.sort();

    for port in ports {
        println!("Port {} is open", port);
    }
}

fn main() {
    let config = Config::from_args();
    run(config);
}
