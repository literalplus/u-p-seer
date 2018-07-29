use pnet::{packet, transport};
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::TransportChannelType::Layer4;
use pnet::packet::ip::IpNextHeaderProtocols::Icmp;
use pnet::packet::icmp::{self, IcmpTypes, MutableIcmpPacket};
use pnet::packet::icmp::echo_request::{MutableEchoRequestPacket, EchoRequestPacket};
use std::net::IpAddr;

pub struct PingOptions {
    pub dest_addr: IpAddr
}

pub struct PingResponse {
    src_addr: IpAddr,
    seq: u16
}

pub fn send_ping(options: &PingOptions) -> Result<PingResponse, String> {
    if !options.dest_addr.is_ipv4() {
        return Err("Only IPv4 is supported so far.".into());
    }

    let (mut tx, mut rx) =
        match transport::transport_channel(4096, Layer4(Ipv4(Icmp))) {
            Ok((tx, rx)) => (tx, rx),
            Err(e) => panic!("Unable to create datalink channel: {}", e)
        };

    let request = make_echo_request();
    print!(" . sending: {:?}", &request);

    match tx.send_to(request, options.dest_addr) {
        Ok(_) => println!("...sent!"),
        Err(e) => panic!("Unable to send: {}", e)
    }

    let mut iter = transport::icmp_packet_iter(&mut rx);

    let res = match iter.next() {
        Ok((packet, addr)) => {
            if addr != options.dest_addr {
                println!(" ! Received ping response from unexpected source: {}", addr);
            }
            if packet.get_icmp_type() != IcmpTypes::EchoReply {
                println!(" ! Unexpected ICMP type: {:?} from {}, data: {:?}", packet.get_icmp_type(), addr, packet)
            }
            println!(" . from {}: {:?}", addr, packet);
            Ok(PingResponse{
                src_addr: addr,
                seq: 1
            })
        }
        Err(e) => Err(format!("Error receiving ICMP packet: {}", e))
    };
    res
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
