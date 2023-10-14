use crate::{constants::DNS_HEADER_BYTES_LENGTH, helpers::extract_bit_from_byte};

const DNS_DATA_ID_LENGTH_START: usize = 0;
const DNS_DATA_ID_LENGTH_END: usize = 1;

const IS_ANSWER_BIT: u8 = 1;
const AUTHORATIVE_BIT: u8 = 6;
const TRUNCATED_BIT: u8 = 7;
const RECURSION_DESIRED_BIT: u8 = 8;

#[derive(Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub is_answer: bool,
    pub opcode: u8,
    pub authorative_answer: bool,
    pub truncated_message: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub reserved: u8,
    pub response_code: u8,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16,
}

impl From<[u8; DNS_HEADER_BYTES_LENGTH]> for DNSHeader {
    fn from(value: [u8; DNS_HEADER_BYTES_LENGTH]) -> Self {
        DNSHeader {
            id: u16::from_ne_bytes(
                value[DNS_DATA_ID_LENGTH_START..=DNS_DATA_ID_LENGTH_END]
                    .try_into()
                    .unwrap(),
            ),
            is_answer: extract_bit_from_byte(IS_ANSWER_BIT, value[2]).unwrap(),
            opcode: 0,
            authorative_answer: extract_bit_from_byte(AUTHORATIVE_BIT, value[2]).unwrap(),
            truncated_message: extract_bit_from_byte(TRUNCATED_BIT, value[2]).unwrap(),
            recursion_desired: extract_bit_from_byte(RECURSION_DESIRED_BIT, value[2]).unwrap(),
            recursion_available: false,
            reserved: 0,
            response_code: 0,
            question_count: 0,
            answer_count: 0,
            authority_count: 0,
            additional_count: 0,
        }
    }
}
