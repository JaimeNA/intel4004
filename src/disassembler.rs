use super::intel4001::Intel4001;
use super::intel4004::Intel4004;

// Desassembler

pub fn print_cpu_state(cpu: &Intel4004) {

    // Program Counter
    println!("\nPC: {} ", cpu.get_pc());

    // Carry
    println!("\nCarry: {}", cpu.get_carry());

    // Accumulator
    println!("\nAccumulator: {:#01X}", cpu.get_acc());

    // Index
    print_index(cpu.get_index());

    // Stack 
    print_stack(cpu.get_stack());
}

pub fn print_index(index: &[u8; 16]) {
    println!("\nIndex: ");

    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", index[0] , index[1] , index[2] , index[3] );
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", index[4] , index[5] , index[6] , index[7] );
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", index[8] , index[9] , index[10], index[11]);
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", index[12], index[13], index[14], index[15]);
}

pub fn print_stack(stack: &[u16; 3]) {
    println!("\nStack:");

    println!(" Level 1: {:#02X}", stack[0]);
    println!(" Level 2: {:#02X}", stack[1]);
    println!(" Level 3: {:#02X}", stack[2]);
}

pub fn print_rom(rom: &Intel4001) {

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