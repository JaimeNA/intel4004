
// Intel 4004

struct Intel4004 {
    registers: Registers,
}

enum OpCodes {

}
    
impl Intel4004 {
    pub fn new() -> Intel4004 {

        let reg = Registers {
            pc: 0x00,
            carry: false,
            acc: 0x00,
            index: [0x00; 16],
            stack: [0x00; 3],
        };

        Intel4004 {
            registers: reg,
        }
    }
}

fn cycle() {

    fetch_instruction();
    decode_op();

}
fn reset() {

}

fn fetch_instruction() {

}

fn decode_op() {
        
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

fn print_registers(r: &Registers) {

    // Program Counter
    println!("PC: {} ", r.pc);

    // Carry
    println!("\nCarry: {}", r.carry);

    // Accumulator
    println!("\nAccumulator: {:#01X}", r.acc);

    // Index
    println!("\nIndex: ");

    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", r.index[0] , r.index[1] , r.index[2] , r.index[3] );
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", r.index[4] , r.index[5] , r.index[6] , r.index[7] );
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", r.index[8] , r.index[9] , r.index[10], r.index[11]);
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", r.index[12], r.index[13], r.index[14], r.index[15]);

    // Stack 
    println!("\nStack:");
    println!(" Level 1: {:#02X}", r.stack[0]);
    println!(" Level 2: {:#02X}", r.stack[1]);
    println!(" Level 3: {:#02X}", r.stack[2]);

    
}

// TODO: add ROM mudule(Intel 4001) and RAM module

fn main() {

    // TODO; Initialize cpu in order to test function
    
    print_registers(&Intel4004::new().registers);
}
