// Intel 4002(RAM)

pub struct Intel4002 {
    pub ram: [u8; 80],       // 320 bits of 4 bit characterrs or 40 bytes
    pub status: [u8; 16],    // 16 status characters
    pub output: u8,          // Output lines
}

impl Intel4002 {
    pub fn new() -> Self {
        Intel4002 {
            ram: [0x00; 80],
            status: [0x00; 16],
            output: 0x00,
        }
    }
}