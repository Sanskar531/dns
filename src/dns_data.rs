use crate::answer::DNSAnswer;
use crate::constants::{DNS_HEADER_BYTES_LENGTH, DNS_DATA_BYTES_LENGTH};
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
        let header_bytes: [u8; DNS_HEADER_BYTES_LENGTH] = buffer
            [..DNS_HEADER_BYTES_LENGTH]
            .try_into()
            .unwrap();

        let mut dns_date = DNSData {
            header: DNSHeader::from(&header_bytes),
            question: DNSQuestion::from(buffer),
            answer: None,
        };
        
        if dns_date.header.is_answer {
            dns_date.answer = Some(DNSAnswer::from(buffer))
        }

        return dns_date;
    }
}

#[allow(dead_code)]
impl DNSData {
    fn new(header: DNSHeader, question: DNSQuestion, answer: DNSAnswer) -> DNSData {
        DNSData {
            header,
            question,
            answer: Some(answer),
        }
    }
}
