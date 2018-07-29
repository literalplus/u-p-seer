extern crate ansi_term;
extern crate oping;
use oping::{Ping, PingResult, PingItem};
use std::fmt::{Display, Formatter};

fn main() {
    println!(" ~ ~ ~ u-p-seer v{} is overseeing your connection ~ ~ ~",
             option_env!("CARGO_PKG_VERSION").unwrap_or("?.?.?"));
    do_example_pings().unwrap();
}

fn do_example_pings() -> PingResult<()> {
    let mut ping = Ping::new();
    ping.set_timeout(1.0)?;
    ping.add_host("localhost")?;
    ping.add_host("10.137.0.10")?;
    ping.add_host("192.168.0.1")?;
    let responses = ping.send()?;
    for response in responses {
        println!("{}", response.hostname);
    }
    return Ok(())
}