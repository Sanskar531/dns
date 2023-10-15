use crate::{
    constants::DNS_HEADER_BYTES_LENGTH,
    helpers::{extract_bit_from_byte, extract_num_from_bits},
};

// Directly copied from the DNS RFC
// Header have specifc byte index and doesn't
// need offset like question and answer part of
// the DNS data.
const DNS_DATA_ID_LENGTH_START: usize = 0;
const DNS_DATA_ID_LENGTH_END: usize = 1;

const OP_CODE_AND_FLAGS_BYTE: usize = 2;
const IS_ANSWER_BIT: u8 = 1;
const OP_CODE_BITS: (u8, u8) = (2, 5);
const AUTHORATIVE_BIT: u8 = 6;
const TRUNCATED_BIT: u8 = 7;
const RECURSION_DESIRED_BIT: u8 = 8;

const RESPONSE_CODE_AND_FLAGS_BYTE: usize = 3;
const RECURSION_AVAILABLE_BIT: u8 = 1;
const RESERVED_BITS: (u8, u8) = (2, 4);
const RESPONSE_CODE_BITS: (u8, u8) = (5, 8);

const DNS_DATA_QUESTION_COUNT_LENGTH_START: usize = 4;
const DNS_DATA_QUESTION_COUNT_LENGTH_END: usize = 5;

const DNS_DATA_ANSWER_COUNT_LENGTH_START: usize = 6;
const DNS_DATA_ANSWER_COUNT_LENGTH_END: usize = 7;

const DNS_DATA_AUTHORITY_COUNT_LENGTH_START: usize = 8;
const DNS_DATA_AUTHORITY_COUNT_LENGTH_END: usize = 9;

const DNS_DATA_ADDITIONAL_COUNT_LENGTH_START: usize = 10;
const DNS_DATA_ADDITIONAL_COUNT_LENGTH_END: usize = 11;

#[derive(Debug,Clone)]
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

impl From<&[u8; DNS_HEADER_BYTES_LENGTH]> for DNSHeader {
    fn from(value: &[u8; DNS_HEADER_BYTES_LENGTH]) -> Self {
        dbg!(value);
        DNSHeader {
            id: u16::from_be_bytes(
                value[DNS_DATA_ID_LENGTH_START..=DNS_DATA_ID_LENGTH_END]
                    .try_into()
                    .unwrap(),
            ),
            is_answer: extract_bit_from_byte(IS_ANSWER_BIT, value[OP_CODE_AND_FLAGS_BYTE]).unwrap(),
            opcode: extract_num_from_bits(OP_CODE_BITS, value[OP_CODE_AND_FLAGS_BYTE]).unwrap(),
            authorative_answer: extract_bit_from_byte(
                AUTHORATIVE_BIT,
                value[OP_CODE_AND_FLAGS_BYTE],
            )
            .unwrap(),
            truncated_message: extract_bit_from_byte(TRUNCATED_BIT, value[OP_CODE_AND_FLAGS_BYTE])
                .unwrap(),
            recursion_desired: extract_bit_from_byte(
                RECURSION_DESIRED_BIT,
                value[OP_CODE_AND_FLAGS_BYTE],
            )
            .unwrap(),
            recursion_available: extract_bit_from_byte(
                RECURSION_AVAILABLE_BIT,
                value[RESPONSE_CODE_AND_FLAGS_BYTE],
            )
            .unwrap(),
            reserved: extract_num_from_bits(RESERVED_BITS, value[RESPONSE_CODE_AND_FLAGS_BYTE])
                .unwrap(),
            response_code: extract_num_from_bits(
                RESPONSE_CODE_BITS,
                value[RESPONSE_CODE_AND_FLAGS_BYTE],
            )
            .unwrap(),
            question_count: u16::from_be_bytes(
                value[DNS_DATA_QUESTION_COUNT_LENGTH_START..=DNS_DATA_QUESTION_COUNT_LENGTH_END]
                    .try_into()
                    .unwrap(),
            ),
            answer_count: u16::from_be_bytes(
                value[DNS_DATA_ANSWER_COUNT_LENGTH_START..=DNS_DATA_ANSWER_COUNT_LENGTH_END]
                    .try_into()
                    .unwrap(),
            ),
            authority_count: u16::from_be_bytes(
                value[DNS_DATA_AUTHORITY_COUNT_LENGTH_START..=DNS_DATA_AUTHORITY_COUNT_LENGTH_END]
                    .try_into()
                    .unwrap(),
            ),
            additional_count: u16::from_be_bytes(
                value
                    [DNS_DATA_ADDITIONAL_COUNT_LENGTH_START..=DNS_DATA_ADDITIONAL_COUNT_LENGTH_END]
                    .try_into()
                    .unwrap(),
            ),
        }
    }
}

impl Into<[u8; DNS_HEADER_BYTES_LENGTH]> for DNSHeader {
    fn into(self) -> [u8; DNS_HEADER_BYTES_LENGTH] {
        let mut bit_encoded_dns_header = [0; DNS_HEADER_BYTES_LENGTH];
        bit_encoded_dns_header[DNS_DATA_ID_LENGTH_START..=DNS_DATA_ID_LENGTH_END]
            .copy_from_slice(&self.id.to_be_bytes());

        let op_code_byte = {
            let mut val = 0u8;
            if self.is_answer {
                val = val | 1;
            }
            val = val << 3 | self.opcode & 0b00000111;
            val = val << 1;
            if self.authorative_answer {
                val = val | 1;
            }
            val = val << 1;
            if self.truncated_message {
                val = val | 1;
            }
            val = val << 1;
            if self.recursion_desired {
                val = val | 1;
            }

            val
        };
        bit_encoded_dns_header[OP_CODE_AND_FLAGS_BYTE] = op_code_byte;

        let response_code_byte = {
            let mut val = 0u8;
            if self.recursion_available {
                val = val | 1;
            }
            val = val << 3 | self.reserved & 0b00000111;
            val = val << 4 | self.response_code & 0b00001111;

            val
        };
        bit_encoded_dns_header[RESPONSE_CODE_AND_FLAGS_BYTE] = response_code_byte;

        let question_count_bytes = self.question_count.to_be_bytes();
        bit_encoded_dns_header
            [DNS_DATA_QUESTION_COUNT_LENGTH_START..=DNS_DATA_QUESTION_COUNT_LENGTH_END]
            .copy_from_slice(&question_count_bytes);

        let answer_count_bytes = self.answer_count.to_be_bytes();
        bit_encoded_dns_header
            [DNS_DATA_ANSWER_COUNT_LENGTH_START..=DNS_DATA_ANSWER_COUNT_LENGTH_END]
            .copy_from_slice(&answer_count_bytes);

        let authority_count_bytes = self.authority_count.to_be_bytes();
        bit_encoded_dns_header
            [DNS_DATA_AUTHORITY_COUNT_LENGTH_START..=DNS_DATA_AUTHORITY_COUNT_LENGTH_END]
            .copy_from_slice(&authority_count_bytes);

        let additonal_count_bytes = self.additional_count.to_be_bytes();
        bit_encoded_dns_header
            [DNS_DATA_ADDITIONAL_COUNT_LENGTH_START..=DNS_DATA_ADDITIONAL_COUNT_LENGTH_END]
            .copy_from_slice(&additonal_count_bytes);

        dbg!(bit_encoded_dns_header);

        bit_encoded_dns_header
    }
}
