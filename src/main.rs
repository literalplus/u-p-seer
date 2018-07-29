extern crate ansi_term;
extern crate pnet;

pub mod net;

use std::net::Ipv4Addr;

fn main() {
    println!(" ~ ~ ~ u-p-seer v{} is overseeing your connection ~ ~ ~",
             option_env!("CARGO_PKG_VERSION").unwrap_or("?.?.?"));
    do_raw_ping();
}

fn do_raw_ping() {
    net::ping::send_ping(&net::ping::PingOptions {
        dest_addr: Ipv4Addr::new(10, 137, 1, 1).into()
    }).unwrap();
}
