use std::{
    io, 
    io::Read,
    fs::File,
};

// Intel 4004(CPU)

struct Intel4004 {
    pc:    u8,
    carry: bool, 
    acc:   u8,
    index: [u8; 16],                                                 // Dynamic RAM cell array of 16 x 4 bits.
    stack: [u16; 3],                                                 // 3 x 12 bits array
    ram_addr: u8,
    rom: Intel4001,                                                  // TODO: do an implementation of a bus or join the application together     
}
  
impl Intel4004 {
    pub fn new() -> Intel4004 {
        Intel4004 {
            pc: 0x00,
            carry: false,
            acc: 0x00,
            index: [0x00; 16],
            stack: [0x00; 3],
            ram_addr: 0x0,
            rom: Intel4001::new(),
        }
    }
  
    pub fn clock(&mut self) {
        self.decode_op(self.rom.fetch_u8(self.pc.into()));
    }
    
    fn reset(&mut self) {
        *self = Intel4004 {
            pc: 0x00,
            carry: false,
            acc: 0x00,
            index: [0x00; 16],
            stack: [0x00; 3],
            ram_addr: 0x0,
            rom: Intel4001::new(),
        };
    }

    // 1-word instructions take 1 instruction cycle while 2-word intructions take 2.
    fn decode_op(&mut self, op_code: u8) {
        match op_code & 0xF0{
            // Machine instructions
            0x00 => self.nop()       ,
            0x10 => self.jnc(op_code & 0x0F), // 2-word instruction
            0x20 => {                  // Check last 4 bit to see which function to call 
                if (op_code & 0x0F) % 2 == 0 {
                    self.fim(op_code & 0x0F);
                } else {
                    self.src(op_code & 0x0F);
                }
            }, 
            0x30 => self.fin(op_code & 0x0F),
            0x31 => self.jin(op_code & 0x0F),
            0x40 => self.jun(op_code & 0x0F), // 2-word instruction
            0x50 => self.jms(op_code & 0x0F), // 2-word instruction
            0x60 => self.inc(op_code & 0x0F),
            0x70 => self.isz(op_code & 0x0F), // 2-word instruction
            0x80 => self.add(op_code & 0x0F),
            0x90 => self.sub(op_code & 0x0F),
            0xA0 => self.ld(op_code & 0x0F),
            0xB0 => self.xch(op_code & 0x0F),
            0xC0 => self.bbl(op_code & 0x0F),
            0xD0 => self.ldm(op_code & 0x0F),
            _ => self.pc += 3            // TODO: handle illegal op codes
        }
    }

    // Instructions

    // A1, A2, A3 cycles are used to request data from the ROM, then M1 and M2 cycles are used to send the data to the CPU. 
    // Finally, the X1, X2 and X3 cycles are used to interpret and execute the instruction.

    // No operation.
    fn nop(&mut self) {
        self.pc += 1;
    }

    // Jump to ROM address X if condition is true, otherwise skip.
    fn jnc(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Fetch immediate from ROM data X to specified index register pair.
    fn fim(&mut self, opa: u8) {

        self.pc += 1;

        self.index[((opa >> 1) * 2) as usize] = self.rom.fetch_u8(self.pc.into());

        self.pc += 1;
    }

    // Send register control. Send the context of the specified index pair to ROM and RAM at set time.
    fn src(&mut self, opa: u8) {

        self.ram_addr = opa >> 1;
        
        self.pc += 1;
    }

    // Fetch indirect from ROM. Send content of index register pair location 0 out as an address. Data fetched is placed in specied register pair.
    fn fin(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Jump indirect. Send content of specified register pair out as an address at A1 and A2 time in the Instruction Cyle. (?)
    fn jin(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Jump unconditional. To specified address.
    fn jun(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Jump to subroutine of specified ROM address, save onl address(Up 1 level in stack).
    fn jms(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Increment contect of specified register.
    fn inc(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Increment contect of specified register. Go to specified ROM address if result != 0, otherwise skip/
    fn isz(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Add contents of specified register to accumulator with carry.
    fn add(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Subtract contents of specified register to accumulator with borrow. 
    fn sub(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Load contents of specified register to accumulator.
    fn ld(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Exchange contents of specified index register and accumulator.
    fn xch(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Branch back (down 1 level in stack) and load specified data to accumulator.
    fn bbl(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Load specified data to accumulator
    fn ldm(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }
}

// Intel 4001(ROM)

struct Intel4001 {
    rom: [u8; 256],      // 256 bytes.
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

    pub fn load_rom(&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        file.read(&mut self.rom).unwrap();
    }
}

// Intel 4002(RAM)

struct Intel4002 {
    ram: [u8; 80],                                                   // 320 bits of 4 bit characterrs or 40 bytes
}

// Desassembler

fn print_cpu_state(cpu: &Intel4004) {

    // Program Counter
    println!("\nPC: {} ", cpu.pc);

    // Carry
    println!("\nCarry: {}", cpu.carry);

    // Accumulator
    println!("\nAccumulator: {:#01X}", cpu.acc);

    // Index
    println!("\nIndex: ");

    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", cpu.index[0] , cpu.index[1] , cpu.index[2] , cpu.index[3] );
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", cpu.index[4] , cpu.index[5] , cpu.index[6] , cpu.index[7] );
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", cpu.index[8] , cpu.index[9] , cpu.index[10], cpu.index[11]);
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", cpu.index[12], cpu.index[13], cpu.index[14], cpu.index[15]);

    // Stack 
    println!("\nStack:");
    println!(" Level 1: {:#02X}", cpu.stack[0]);
    println!(" Level 2: {:#02X}", cpu.stack[1]);
    println!(" Level 3: {:#02X}", cpu.stack[2]);

    
}

fn print_rom(rom: &Intel4001) {

    let mut i = 0;

    println!("\nROM: ");

    println!("0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");

    for inst in rom.rom {
        print!("{:02X} ", inst);

        i += 1;

        if i == 16 {
            println!("");
            i = 0;
        }
    }

}

fn main() -> io::Result<()>{

    let mut cpu = Intel4004::new();

    cpu.rom.load_rom("rom/RDn");

    cpu.clock();
    cpu.clock();

    print_rom(&cpu.rom);
    print_cpu_state(&cpu);

    Ok(())
}
