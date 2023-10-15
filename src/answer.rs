use std::net::{IpAddr, Ipv4Addr};

use crate::{
    constants::DNS_DATA_BYTES_LENGTH, helpers::DNSBodyParser, question::DNS_QUESTION_START_BYTE,
};

const DNS_QUESTION_REMAINING_BYTES: usize = 4;
const DNS_QUERY_OFFSET_BYTES: usize = 2;

const DNS_ANSWER_TYPE_LENGTH: usize = 2;
const DNS_ANSWER_CLASS_LENGTH: usize = 2;

const DNS_ANSWER_TTL_LENGTH: usize = 4;
const DNS_ANSWER_LEN_LENGTH: usize = 2;

#[allow(dead_code)]
#[derive(Debug)]
pub struct AnswerPreamble {
    pub question: String,
    pub record_type: u16,
    pub class: u16,
    pub ttl: u32,
    pub len: u16,
}

#[derive(Debug)]
pub struct DNSAnswer {
    pub preamble: AnswerPreamble,
    pub ip: Ipv4Addr,
}

impl From<&[u8; DNS_DATA_BYTES_LENGTH]> for DNSAnswer {
    fn from(value: &[u8; DNS_DATA_BYTES_LENGTH]) -> Self {
        let question_body = &value[DNS_QUESTION_START_BYTE..];
        let (question, mut question_ending_idx) =
            DNSBodyParser::extract_body_string(question_body).unwrap();
        question_ending_idx += DNS_QUESTION_START_BYTE + DNS_QUESTION_REMAINING_BYTES;

        let ending_idx = question_ending_idx + DNS_QUERY_OFFSET_BYTES;
        let ip_addr: [u8; 4] = value[(ending_idx
            + DNS_ANSWER_TYPE_LENGTH
            + DNS_ANSWER_CLASS_LENGTH
            + DNS_ANSWER_TTL_LENGTH
            + DNS_ANSWER_LEN_LENGTH)
            ..(ending_idx
                + DNS_ANSWER_TYPE_LENGTH
                + DNS_ANSWER_CLASS_LENGTH
                + DNS_ANSWER_TTL_LENGTH
                + DNS_ANSWER_LEN_LENGTH
                + 4)]
            .try_into()
            .unwrap();
        DNSAnswer {
            preamble: AnswerPreamble {
                question,
                class: u16::from_be_bytes(
                    value[ending_idx..(ending_idx + DNS_ANSWER_TYPE_LENGTH)]
                        .try_into()
                        .unwrap(),
                ),
                record_type: u16::from_be_bytes(
                    value[(ending_idx + DNS_ANSWER_TYPE_LENGTH)
                        ..(ending_idx + DNS_ANSWER_TYPE_LENGTH + DNS_ANSWER_CLASS_LENGTH)]
                        .try_into()
                        .unwrap(),
                ),
                ttl: u32::from_be_bytes(
                    value[(ending_idx + DNS_ANSWER_TYPE_LENGTH + DNS_ANSWER_CLASS_LENGTH)
                        ..(ending_idx
                            + DNS_ANSWER_TYPE_LENGTH
                            + DNS_ANSWER_CLASS_LENGTH
                            + DNS_ANSWER_TTL_LENGTH)]
                        .try_into()
                        .unwrap(),
                ),
                len: u16::from_be_bytes(
                    value[(ending_idx
                        + DNS_ANSWER_TYPE_LENGTH
                        + DNS_ANSWER_CLASS_LENGTH
                        + DNS_ANSWER_TTL_LENGTH)
                        ..(ending_idx
                            + DNS_ANSWER_TYPE_LENGTH
                            + DNS_ANSWER_CLASS_LENGTH
                            + DNS_ANSWER_TTL_LENGTH
                            + DNS_ANSWER_LEN_LENGTH)]
                        .try_into()
                        .unwrap(),
                ),
            },
            ip: Ipv4Addr::new(ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3]),
        }
    }
}
