#[allow(dead_code)]
pub struct DNSHeader {
    id: u16,
    is_query: bool,
    opcode: u8,
    authorative_answer: bool,
    truncated_message: bool,
    recursion_desired: bool,
    recursion_available: bool,
    reserved: u8,
    response_code: u8,
    question_count: u16,
    answer_count: u16,
    authority_count: u16,
    additional_count: u16,
}
