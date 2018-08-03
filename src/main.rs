extern crate ansi_term;
extern crate oping;
extern crate term;
extern crate floating_duration;

pub mod net;

use std::io::{self, Write};
use std::sync::mpsc::channel;
use std::time::Duration;
use net::api::{PingConfig, PingResult, PingBackend};
use std::time::Instant;
use std::thread;

fn main() {
    println!(" ~ ~ ~ u-p-seer v{} is overseeing your connection ~ ~ ~",
             option_env!("CARGO_PKG_VERSION").unwrap_or("?.?.?"));
    let ping_hosts = find_ping_hosts();
    let cfg = PingConfig {
        addrs: ping_hosts,
        timeout: Duration::from_millis(700),
    };
    let mut ping = net::oping::OPingBackend::new(cfg);
    ping.prepare().unwrap();
    loop {
        let start_is = Instant::now();
        let (tx, rx) = channel();
        ping.send(tx);
        while let Ok(res) = rx.recv() {
            match res as PingResult {
                Ok(resp) => {
                    println!(" + {} responded in {}ms",
                             resp.get_request_address(), resp.get_request_address());
                }
                Err(err) => {
                    println!(" * {} is dead: {}",
                             err.address.unwrap_or("???".into()), err.message);
                }
            }
        }
        if start_is.elapsed().as_secs() < 1 { // ensure ping interval >= 1s
            thread::sleep(Duration::from_secs(1) - start_is.elapsed());
        }
    }
}

fn find_ping_hosts() -> Vec<String> {
    let mut ping_hosts: Vec<String> = std::env::args().skip(1).collect();
    if ping_hosts.is_empty() {
        println!(" No hosts to ping, add some: (empty line means done)");
        let stdin = io::stdin();
        loop {
            print!(" - ");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            {
                stdin.read_line(&mut buf).unwrap();
            }
            if buf.trim().is_empty() {
                let mut term = term::stdout().unwrap();
                term.carriage_return().unwrap();
                term.cursor_up().unwrap();
                break;
            } else {
                ping_hosts.push(buf.trim().into());
            }
        }
        if ping_hosts.is_empty() {
            panic!("Still no ping hosts.");
        }
    } else {
        for arg in std::env::args().skip(1) {
            println!("  - {}", arg);
        }
    }
    ping_hosts
}