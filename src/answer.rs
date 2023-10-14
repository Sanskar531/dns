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
    pub ip: u32,
}
