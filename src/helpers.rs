pub const BYTE_SIZE_IN_BITS: u8 = 8;

pub fn extract_bit_from_byte(bit_idx: u8, num: u8) -> Result<bool, &'static str> {
    if bit_idx > BYTE_SIZE_IN_BITS {
        return Err("bit_idx is greater than the number of bits in a byte.");
    }

    // Shift the bit to the left most side and see if it is 1.
    println!("{:b}", num);
    Ok((num >> (BYTE_SIZE_IN_BITS - bit_idx)) == 1)
}
