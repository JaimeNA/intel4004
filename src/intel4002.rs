// Intel 4002(RAM)

pub struct Intel4002 {
    pub ram: [u8; 64],       // 64 4-bits characters.
    pub status: [u8; 16],    // 16 status characters.
    pub output: u8,          // Output lines.
}

impl Intel4002 {
    pub fn new() -> Self {
        Intel4002 {
            ram: [0x00; 64],
            status: [0x00; 16],
            output: 0x00,
        }
    }
}