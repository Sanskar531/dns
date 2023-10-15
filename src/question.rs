use crate::{constants::DNS_DATA_BYTES_LENGTH, helpers::DNSBodyParser};

pub const DNS_QUESTION_START_BYTE: usize = 12;

const DNS_QUESTION_TYPE_LENGTH: usize = 2;
const DNS_QUESTION_CLASS_LENGTH: usize = 2;

#[derive(Debug)]
pub struct DNSQuestion {
    pub question: String,
    pub record_type: u16,
    pub class: u16,
}

impl From<&[u8; DNS_DATA_BYTES_LENGTH]> for DNSQuestion {
    fn from(value: &[u8; DNS_DATA_BYTES_LENGTH]) -> Self {
        let body = &value[DNS_QUESTION_START_BYTE..];
        let (question, mut ending_idx) = DNSBodyParser::extract_body_string(body).unwrap();
        ending_idx += DNS_QUESTION_START_BYTE;

        DNSQuestion {
            question,
            class: u16::from_be_bytes(
                value[ending_idx..(ending_idx + DNS_QUESTION_TYPE_LENGTH)]
                    .try_into()
                    .unwrap(),
            ),
            record_type: u16::from_be_bytes(
                value[(ending_idx + DNS_QUESTION_TYPE_LENGTH)
                    ..(ending_idx + DNS_QUESTION_TYPE_LENGTH + DNS_QUESTION_CLASS_LENGTH)]
                    .try_into()
                    .unwrap(),
            ),
        }
    }
}
