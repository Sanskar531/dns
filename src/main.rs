use std::fs;

use crate::packet::DNSPacketBuffer;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("/home/sanskar/query_packet.txt");
    if let Ok(a) = contents {
        println!("{}", a);
        let buffer: DNSPacketBuffer = DNSPacketBuffer::from(a);
    }
}

pub mod dns_header;
pub mod question;
pub mod answer;
pub mod packet;
