use crate::constants::DNS_DATA_BYTES_LENGTH;

pub const BYTE_SIZE_IN_BITS: u8 = 8;

pub fn extract_bit_from_byte(bit_idx: u8, num: u8) -> Result<bool, &'static str> {
    if bit_idx > BYTE_SIZE_IN_BITS {
        return Err("bit_idx is greater than the number of bits in a byte.");
    }

    // Shift the bit to the left most side and see if it is 1.
    Ok((num >> (BYTE_SIZE_IN_BITS - bit_idx)) == 1)
}

pub fn extract_num_from_bits(bit_idx: (u8, u8), num: u8) -> Result<u8, &'static str> {
    if bit_idx.0 > BYTE_SIZE_IN_BITS || bit_idx.1 > BYTE_SIZE_IN_BITS {
        return Err("bit_idx is greater than the number of bits in a byte.");
    }

    if bit_idx.0 > bit_idx.1 {
        return Err("bid_idx at 0 should be smaller than bid_idx at 1.");
    }
    let mut value: u8 = 0;

    // Shift the bit to the left most side and see if it is 1.
    for i in bit_idx.0..=bit_idx.1 {
        if i != bit_idx.0 {
            value = value << 1;
        }
        let bit = extract_bit_from_byte(i, num).unwrap();
        // If bit is set
        if bit {
            value = value | 1;
        }
    }

    Ok(value)
}

pub struct DNSBodyParser {}

impl DNSBodyParser {
    pub fn extract_body_string(body_buf: &[u8]) -> Result<(String, usize), &'static str> {
        let mut value = String::new();
        let mut ending_idx: Option<usize> = None;

        for i in 0..body_buf.len() {
            if body_buf[i] == 0 {
                ending_idx = Some(i + 1);
                break;
            }

            if body_buf[i].is_ascii_alphabetic() {
                value.push(body_buf[i].to_ascii_lowercase() as char);
            } else {
                value += ".";
            }
        }

        if let Some(idx) = ending_idx {
            if idx == DNS_DATA_BYTES_LENGTH {
                Err("Index out of range.")
            } else {
                Ok((value.into(), idx))
            }
        } else {
            Err("Couldn't find ending index.")
        }
    }
}
