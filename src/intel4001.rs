use std::{
    io, 
    io::Read,
    fs::File,
};

// Intel 4001(ROM)

pub struct Intel4001 {
    pub rom: [u8; 256],      // 256 bytes.
    pub io: u8,                  // 4 bits I/O port to route data in and out of the system.
}

impl Intel4001 {
    pub fn new() -> Self {
        Intel4001 {
            rom: [0x00; 256],
            io: 0x00,
        }
    }

    pub fn fetch_u8(&self, addr: usize) -> u8{

        if addr < 256 {
            return self.rom[addr];
        }
        0x00
    }

    pub fn load_rom(&mut self, filename: &str) -> io::Result<()>{
        let mut file = File::open(filename)?;
        file.read(&mut self.rom)?;

        Ok(())
    }
}