use std::{
    io, 
    io::Read,
    fs::File,
};

// Intel 4001(ROM)

pub struct Intel4001 {
    pub rom: [u8; 256],      // 256 bytes.
}

impl Intel4001 {
    pub fn new() -> Intel4001 {
        Intel4001 {
            rom: [0x00; 256],
        }
    }

    pub fn fetch_u8(&self, addr: usize) -> u8{
        self.rom[addr]
    }

    pub fn load_rom(&mut self, filename: &str) -> io::Result<()>{
        let mut file = File::open(filename)?;
        file.read(&mut self.rom)?;

        Ok(())
    }
}