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
}
  
impl Intel4004 {
    pub fn new() -> Intel4004 {
        Intel4004 {
            pc: 0x00,
            carry: false,
            acc: 0x00,
            index: [0x00; 16],
            stack: [0x00; 3],
        }
    }

    // TODO: Implement clock function
    
    fn reset(&mut self) {
        *self = Intel4004 {
            pc: 0x00,
            carry: false,
            acc: 0x00,
            index: [0x00; 16],
            stack: [0x00; 3],
        };
    }

    fn fetch_instruction() {

    }

    // 1-word instructions take 1 instruction cycle while 2-word intructions take 2.
    fn decode_op(&mut self, op_code: u16) {
        match op_code{
            // Machine instructions
            0x0000 => self.nop()       ,
            0x1000 => self.jnc(op_code), // 2-word instruction
            0x2000 => self.fim(op_code), // 2-word instruction 
            0x2100 => self.src(op_code),
            0x3000 => self.fin(op_code),
            0x3100 => self.jin(op_code),
            0x4000 => self.jun(op_code), // 2-word instruction
            0x5000 => self.jms(op_code), // 2-word instruction
            0x6000 => self.inc(op_code),
            0x7000 => self.isz(op_code), // 2-word instruction
            0x8000 => self.add(op_code),
            0x9000 => self.sub(op_code),
            0xA000 => self.ld(op_code),
            0xB000 => self.xch(op_code),
            0xC000 => self.bbl(op_code),
            0xD000 => self.ldm(op_code),
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
    fn jnc(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Fetch immediate from ROM data X to specified index register pair.
    fn fim(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Send register control. Send the context of the specified index pair to ROM and RAM at set time.
    fn src(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Fetch indirect from ROM. Send content of index register pair location 0 out as an address. Data fetched is placed in specied register pair.
    fn fin(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Jump indirect. Send content of specified register pair out as an address at A1 and A2 time in the Instruction Cyle. (?)
    fn jin(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Jump unconditional. To specified address.
    fn jun(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Jump to subroutine of specified ROM address, save onl address(Up 1 level in stack).
    fn jms(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Increment contect of specified register.
    fn inc(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Increment contect of specified register. Go to specified ROM address if result != 0, otherwise skip/
    fn isz(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Add contents of specified register to accumulator with carry.
    fn add(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Subtract contents of specified register to accumulator with borrow. 
    fn sub(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Load contents of specified register to accumulator.
    fn ld(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Exchange contents of specified index register and accumulator.
    fn xch(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Branch back (down 1 level in stack) and load specified data to accumulator.
    fn bbl(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Load specified data to accumulator
    fn ldm(&mut self, op_code: u16) {

        // TODO: internal magic

        self.pc += 1;
    }
}

// Intel 4001(ROM)

struct Intel4001 {
    rom: [u8; 256],      // 256 bytes
}

impl Intel4001 {
    pub fn new() -> Intel4001 {
        Intel4001 {
            rom: [0x00; 256]
        }
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

    let mut rom = Intel4001::new();

    rom.load_rom("rom/RDn");
    print_rom(&rom);

    cpu.decode_op(rom.rom[cpu.pc as usize].into());
    print_cpu_state(&cpu);

    Ok(())
}
