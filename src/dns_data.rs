use crate::answer::{AnswerPreamble, DNSAnswer};
use crate::constants::DNS_HEADER_BYTES_LENGTH;
use crate::header::DNSHeader;
use crate::question::DNSQuestion;

#[derive(Debug)]
pub struct DNSData {
    pub header: DNSHeader,
    pub question: DNSQuestion,
    pub answer: DNSAnswer,
}

impl From<[u8; 512]> for DNSData {
    fn from(buffer_as_string: [u8; 512]) -> Self {
        let header_bytes: [u8; DNS_HEADER_BYTES_LENGTH] = buffer_as_string
            [..DNS_HEADER_BYTES_LENGTH]
            .try_into()
            .unwrap();

        DNSData {
            header: DNSHeader::from(header_bytes),
            question: DNSQuestion {
                record_type: 0,
                class: 0,
                question: "".into(),
            },
            answer: DNSAnswer {
                preamble: AnswerPreamble {
                    question: "".into(),
                    class: 0,
                    record_type: 0,
                    len: 0,
                    ttl: 0,
                },
                ip: 0,
            },
        }
    }
}

#[allow(dead_code)]
impl DNSData {
    fn new(header: DNSHeader, question: DNSQuestion, answer: DNSAnswer) -> DNSData {
        DNSData {
            header,
            question,
            answer,
        }
    }
}
