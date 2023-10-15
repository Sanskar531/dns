use crate::answer::DNSAnswer;
use crate::constants::{DNS_DATA_BYTES_LENGTH, DNS_HEADER_BYTES_LENGTH};
use crate::header::DNSHeader;
use crate::question::DNSQuestion;

#[derive(Debug)]
pub struct DNSData {
    pub header: DNSHeader,
    pub question: DNSQuestion,
    pub answer: Option<DNSAnswer>,
}

impl From<&[u8; DNS_DATA_BYTES_LENGTH]> for DNSData {
    fn from(buffer: &[u8; DNS_DATA_BYTES_LENGTH]) -> Self {
        let header_bytes: [u8; DNS_HEADER_BYTES_LENGTH] =
            buffer[..DNS_HEADER_BYTES_LENGTH].try_into().unwrap();

        let mut dns_data = DNSData {
            header: DNSHeader::from(&header_bytes),
            question: DNSQuestion::from(buffer),
            answer: None,
        };

        if dns_data.header.is_answer {
            dns_data.answer = Some(DNSAnswer::from(buffer))
        }

        let bit_encoded_header: [u8; DNS_HEADER_BYTES_LENGTH] = dns_data.header.clone().into();

        dbg!(bit_encoded_header);

        return dns_data;
    }
}

impl Into<[u8; DNS_DATA_BYTES_LENGTH]> for DNSData {
    fn into(self) -> [u8; DNS_DATA_BYTES_LENGTH] {
        [0; DNS_DATA_BYTES_LENGTH]
    }
}

#[allow(dead_code)]
impl DNSData {
    fn new(header: DNSHeader, question: DNSQuestion, answer: Option<DNSAnswer>) -> DNSData {
        DNSData {
            header,
            question,
            answer,
        }
    }
}
