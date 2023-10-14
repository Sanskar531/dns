use crate::dns_data::DNSData;
use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut buf: [u8; 512] = [0; 512];
    let mut file = File::open("/home/sanskar/query_packet.txt").unwrap();
    file.read(&mut buf)?;

    let buffer: DNSData = DNSData::from(buf);
    println!("{:#?}", buffer);
    Ok(())
}

pub mod answer;
pub mod dns_data;
pub mod header;
pub mod question;
pub mod constants;
pub mod helpers;
