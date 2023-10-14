#[allow(dead_code)]
pub struct DNSPacketBuffer {
    buffer: [u8; 512],
}

impl From<String> for DNSPacketBuffer {
    fn from(buffer_as_string: String) -> DNSPacketBuffer {
        let byte_representation = buffer_as_string.into_bytes();
        println!("{:#?}", byte_representation);
        DNSPacketBuffer {
            buffer: [0; 512],
        }
    }
}

impl From<[u8; 512]> for DNSPacketBuffer {
    fn from(buffer: [u8; 512]) -> DNSPacketBuffer {
        DNSPacketBuffer {
            buffer,
        }
    }
}
