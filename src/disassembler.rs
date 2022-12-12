use super::intel4001::Intel4001;
use super::intel4002::Intel4002;
use super::intel4004::Intel4004;

use arbitrary_int::{u4};

// Disassembler

pub fn print_cpu_state(cpu: &Intel4004) {

    // Program Counter
    println!("\nPC: {} ", cpu.get_pc());

    // Carry
    println!("\nCarry: {}", cpu.get_carry());

    // Accumulator
    println!("\nAccumulator: {:#0X}", cpu.get_acc());

    // Index
    print_index(cpu.get_index());

    // Stack 
    print_stack(cpu.get_stack());
}

pub fn print_index(index: &[u4; 16]) {
    println!("\nIndex: ");

    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", index[0].value() , index[1].value() , index[2].value() , index[3].value() );
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", index[4].value() , index[5].value() , index[6].value() , index[7].value() );
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", index[8].value() , index[9].value() , index[10].value(), index[11].value());
    println!(" {:#01X} {:#01X} {:#01X} {:#01X}", index[12].value(), index[13].value(), index[14].value(), index[15].value());
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

pub fn print_ram(ram: &Intel4002) {
    let mut i = 0;

    println!("\nRAM: ");

    println!("\nMain memory: ");
    println!("0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");

    for inst in ram.ram {
        print!("{:02X} ", inst);

        i += 1;

        if i == 16 {
            println!("");
            i = 0;
        }
    }

    println!("\nStatus characters: ");
    println!("0  1  2  3");

    for inst in ram.status {
        print!("{:02X} ", inst);

        i += 1;

        if i == 4 {
            println!("");
            i = 0;
        }
    }
}