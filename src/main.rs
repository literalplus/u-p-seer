extern crate ansi_term;
extern crate pnet;

use pnet::transport;
use pnet::packet;
use transport::TransportProtocol::Ipv4;
use transport::TransportChannelType::Layer4;
use packet::ip::IpNextHeaderProtocols::Icmp;
use pnet::packet::icmp::{self, IcmpTypes, MutableIcmpPacket};
use icmp::echo_request::{MutableEchoRequestPacket, EchoRequestPacket};
use std::net::Ipv4Addr;

fn main() {
    println!(" ~ ~ ~ u-p-seer v{} is overseeing your connection ~ ~ ~",
             option_env!("CARGO_PKG_VERSION").unwrap_or("?.?.?"));
    do_raw_ping();
}

fn do_raw_ping() {
    let (mut tx, mut rx) =
        match transport::transport_channel(4096, Layer4(Ipv4(Icmp))) {
            Ok((tx, rx)) => (tx, rx),
            Err(e) => panic!("Unable to create datalink channel: {}", e)
        };

//    let dest_ip = Ipv4Addr::new(192, 168, 0, 1);
    let dest_ip = Ipv4Addr::new(10, 137, 1, 1);
    let request = make_echo_request();
    print!(" . sending: {:?}", &request);

    match tx.send_to(request, dest_ip.into()) {
        Ok(_) => println!("...sent!"),
        Err(e) => panic!("Unable to send: {}", e)
    }

    let mut iter = transport::icmp_packet_iter(&mut rx);

    match iter.next() {
        Ok((packet, addr)) => {
            if addr != dest_ip {
                println!(" ! Received ping response from unexpected source: {}", addr);
            }
            if packet.get_icmp_type() != IcmpTypes::EchoReply {
                println!(" ! Unexpected ICMP type: {:?} from {}, data: {:?}", packet.get_icmp_type(), addr, packet)
            }
            println!(" . from {}: {:?}", addr, packet)
        }
        Err(e) => println!("Error receiving ICMP packet: {}", e)
    }
}

fn make_echo_request<'p>() -> EchoRequestPacket<'p> {
    let payload = "u-p-seer".as_bytes();
    let req_size = MutableEchoRequestPacket::minimum_packet_size() + payload.len();
    let mut packet: Vec<u8> = vec![0 as u8; req_size];
    build_echo_request(payload, &mut packet);
    build_icmp_checksum(&mut packet);
    return EchoRequestPacket::owned(packet).unwrap();
}

fn build_icmp_checksum(mut packet: &mut Vec<u8>) {
    let mut icmp_packet = MutableIcmpPacket::new(&mut packet).unwrap();
    let checksum = icmp::checksum(&icmp_packet.to_immutable());
    icmp_packet.set_checksum(checksum);
}

fn build_echo_request(payload: &[u8], mut packet: &mut Vec<u8>) {
    let mut req = MutableEchoRequestPacket::new(&mut packet).unwrap();
    req.set_icmp_type(packet::icmp::IcmpTypes::EchoRequest);
    req.set_payload(payload);
    req.set_sequence_number(1);
}
