
// Intel 4004

struct Intel4004 {
    pc:    u8,
    carry: bool, 
    acc:   u8,
    index: [u8; 16],      // Dynamic RAM cell array of 16 x 4 bits.
    stack: [u16; 3],     // 3 x 12 bits array
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
            0x3000 => self.src(op_code),
            _ => self.pc += 3
        }
    }

    // Instructions

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
}



// Intel 4004 registers 

struct Registers {
    pc:    u8,
    carry: bool, 
    acc:   u8,
    index: [u8; 16],      // Dynamic RAM cell array of 16 x 4 bits.
    stack: [u16; 3],     // 3 x 12 bits array
}


// Desassembler

fn print_cpu_state(cpu: &Intel4004) {

    // Program Counter
    println!("PC: {} ", cpu.pc);

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

// TODO: add ROM mudule(Intel 4001) and RAM module

fn main() {

    let mut cpu = Intel4004::new();

    cpu.decode_op(0x1000);
    print_cpu_state(&cpu);


}
