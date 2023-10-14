#[allow(dead_code)]
pub struct AnswerPreamble {
    question: String,
    record_type: u16,
    class: u16,
    ttl: u32,
    len: u16,
}

#[allow(dead_code)]
pub struct DNSAnswer {
    preamble: AnswerPreamble,
    ip: u32,
}
