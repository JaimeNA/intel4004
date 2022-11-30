
// Intel 4004

struct Intel4004 {
    registers: Registers,
}

enum OpCodes {

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
    index: [u8; 16],      // Dynamic RAM cell array of 16 x 4 bits.
    acc:   u8,
    pc:    u8,
    stack: [u16; 3],     // 3 x 12 bits array
}


// Desassembler

fn print_registers(r: &Registers) {

    // Print stack 
    println!("Stack: /n Level 1: {:#02X} /n Level 2: {:#02X} /n Level 3: {:#02X}", r.stack[0], r.stack[1], r.stack[2] );

}

// TODO: add ROM mudule(Intel 4001) and RAM module

fn main() {

    // TODO; Initialize cpu in order to test function
    
    //print_registers();
}
