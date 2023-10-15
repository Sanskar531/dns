use std::net::Ipv4Addr;

use crate::{
    constants::DNS_DATA_BYTES_LENGTH, helpers::DNSBodyParser, question::DNS_QUESTION_START_BYTE,
};

const DNS_QUESTION_REMAINING_BYTES: usize = 4;
const DNS_QUERY_OFFSET_BYTES: usize = 2;

const DNS_ANSWER_TYPE_LENGTH: usize = 2;
const DNS_ANSWER_CLASS_LENGTH: usize = 2;

const DNS_ANSWER_TTL_LENGTH: usize = 4;
const DNS_ANSWER_LEN_LENGTH: usize = 2;

const DNS_ANSWER_IP_LENGTH: usize = 4;

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
        let mut current_idx = question_ending_idx + DNS_QUERY_OFFSET_BYTES;

        DNSAnswer {
            preamble: AnswerPreamble {
                question,
                class: u16::from_be_bytes(
                    value[current_idx..({
                        current_idx += DNS_ANSWER_TYPE_LENGTH;
                        current_idx
                    })]
                        .try_into()
                        .unwrap(),
                ),
                record_type: u16::from_be_bytes(
                    value[current_idx..({
                        current_idx += DNS_ANSWER_CLASS_LENGTH;
                        current_idx
                    })]
                        .try_into()
                        .unwrap(),
                ),
                ttl: u32::from_be_bytes(
                    value[current_idx..({
                        current_idx += DNS_ANSWER_TTL_LENGTH;
                        current_idx
                    })]
                        .try_into()
                        .unwrap(),
                ),
                len: u16::from_be_bytes(
                    value[current_idx..({
                        current_idx += DNS_ANSWER_LEN_LENGTH;
                        current_idx
                    })]
                        .try_into()
                        .unwrap(),
                ),
            },
            ip: {
                let ip_addr: [u8; 4] = value[current_idx..({
                    current_idx += DNS_ANSWER_IP_LENGTH;
                    current_idx
                })]
                    .try_into()
                    .unwrap();

                Ipv4Addr::new(ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3])
            },
        }
    }
}

impl DNSAnswer {
    fn pack_into(&self, dns_data: &mut [u8; DNS_DATA_BYTES_LENGTH]) {
        // Just pack into question_data section no need to worry about
        // headers
        let question_data = &mut dns_data[DNS_QUESTION_START_BYTE..];

        let split_label: Vec<&str> = self.question.split(".").collect();

        let mut encoded_labels: Vec<u8> = vec![];

        for label in split_label {
            let label_len: u8 = label.len().try_into().unwrap();
            encoded_labels.push(label_len);
            label
                .to_ascii_lowercase()
                .chars()
                .for_each(|current_char| encoded_labels.push(current_char as u8));
        }

        question_data.copy_from_slice(&encoded_labels.as_slice());
    }
}
