#[allow(dead_code)]
pub struct DNSQuestion {
    question: String,
    record_type: u16,
    class: u16,
}
