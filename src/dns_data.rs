use crate::answer::{AnswerPreamble, DNSAnswer};
use crate::header::DNSHeader;
use crate::question::DNSQuestion;

#[derive(Debug)]
pub struct DNSData {
    pub header: DNSHeader,
    pub question: DNSQuestion,
    pub answer: DNSAnswer,
}

impl From<String> for DNSData {
    fn from(buffer_as_string: String) -> DNSData {
        let byte_representation = buffer_as_string.into_bytes();
        println!("{:#?}", byte_representation);
        DNSData {
            header: DNSHeader {
                id: 0,
                is_query: true,
                opcode: 0,
                authorative_answer: false,
                truncated_message: false,
                recursion_desired: false,
                recursion_available: false,
                reserved: 0,
                response_code: 0,
                question_count: 0,
                answer_count: 0,
                authority_count: 0,
                additional_count: 0,
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
            question: DNSQuestion {
                record_type: 0,
                class: 0,
                question: "".into(),
            },
        }
    }
}
