use std::fs;
use crate::dns_data::DNSData;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("/home/sanskar/query_packet.txt");
    if let Ok(query) = contents {
        let buffer: DNSData = DNSData::from(query);
        println!("{:#?}", buffer);
    }
}

pub mod header;
pub mod question;
pub mod answer;
pub mod dns_data;
