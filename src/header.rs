#[allow(dead_code)]
#[derive(Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub is_query: bool,
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
