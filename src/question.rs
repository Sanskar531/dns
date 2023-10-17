use std::io::Write;

use crate::{constants::DNS_DATA_BYTES_LENGTH, helpers::DNSBodyParser};

pub const DNS_QUESTION_START_BYTE: usize = 12;

const DNS_QUESTION_TYPE_LENGTH: usize = 2;
const DNS_QUESTION_CLASS_LENGTH: usize = 2;

#[derive(Debug, Clone)]
pub struct DNSQuestion {
    pub question: String,
    pub record_type: u16,
    pub class: u16,
}

impl From<&[u8; DNS_DATA_BYTES_LENGTH]> for DNSQuestion {
    fn from(value: &[u8; DNS_DATA_BYTES_LENGTH]) -> Self {
        let body = &value[DNS_QUESTION_START_BYTE..];
        dbg!(value);
        let (question, mut current_idx) = DNSBodyParser::extract_body_string(body).unwrap();
        current_idx += DNS_QUESTION_START_BYTE;

        DNSQuestion {
            question,
            class: u16::from_be_bytes(
                value[current_idx..({
                    current_idx += DNS_QUESTION_TYPE_LENGTH;
                    current_idx
                })]
                    .try_into()
                    .unwrap(),
            ),
            record_type: u16::from_be_bytes(
                value[current_idx..({
                    current_idx += DNS_QUESTION_CLASS_LENGTH;
                    current_idx
                })]
                    .try_into()
                    .unwrap(),
            ),
        }
    }
}

impl DNSQuestion {
    // Returns the index at which the question byte ends
    pub fn pack_into(&self, dns_data: &mut [u8; DNS_DATA_BYTES_LENGTH]) -> usize {
        // Just pack into question_data section no need to worry about
        // headers
        let mut question_data = &mut dns_data[DNS_QUESTION_START_BYTE..];

        let split_label: Vec<&str> = self.question.split(".").collect();

        let mut question_labels_and_flags: Vec<u8> = vec![];

        for label in split_label {
            let label_len: u8 = label.len().try_into().unwrap();
            question_labels_and_flags.push(label_len);
            label
                .to_ascii_lowercase()
                .chars()
                .for_each(|current_char| question_labels_and_flags.push(current_char as u8));
        }

        // Since we are finished we need to delimit by 0
        question_labels_and_flags.push(0);

        question_labels_and_flags.extend_from_slice(&self.class.to_be_bytes());
        question_labels_and_flags.extend_from_slice(&self.record_type.to_be_bytes());

        question_data
            .write_all(&question_labels_and_flags.as_slice())
            .unwrap();

        return question_labels_and_flags.len();
    }
}
