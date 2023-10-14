#[allow(dead_code)]
#[derive(Debug)]
pub struct DNSQuestion {
    pub question: String,
    pub record_type: u16,
    pub class: u16,
}
