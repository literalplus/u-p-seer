extern crate ansi_term;
extern crate oping;
extern crate term;

use oping::{Ping, PingResult};
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!(" ~ ~ ~ u-p-seer v{} is overseeing your connection ~ ~ ~",
             option_env!("CARGO_PKG_VERSION").unwrap_or("?.?.?"));
    let ping_hosts = find_ping_hosts();
    loop {
        let ping = mkping(&ping_hosts).unwrap();
        let start_is = Instant::now();
        do_ping(ping).unwrap();
        if start_is.elapsed().as_secs() < 1 {
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

fn mkping(addrs: &Vec<String>) -> PingResult<Ping> {
    let mut ping = Ping::new();
    ping.set_timeout(0.7)?;
    for addr in addrs {
        ping.add_host(&addr)?;
    }
    Ok(ping)
}

fn do_ping(ping: Ping) -> PingResult<()> {
    let responses = ping.send()?;
    for response in responses {
        match response.dropped {
            0 => println!(" + {} in {}ms", response.hostname, response.latency_ms),
            _ => println!(" * {} is rip", response.address)
        }
    }
    return Ok(());
}