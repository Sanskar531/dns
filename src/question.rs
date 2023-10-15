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
