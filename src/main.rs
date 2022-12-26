use std::io;
use std::{thread, time};

use intel4004_emu::intel4004::Intel4004;
use intel4004_emu::disassembler::*;

fn main() -> io::Result<()>{

    let mut cpu = Intel4004::new();

    cpu.rom.load_rom("rom/ram_test")?;

    let mut i = 0;
    
    let delay = time::Duration::from_millis(10);
    let mut now = time::Instant::now();

    while i < 5000 {
        now = time::Instant::now();

        cpu.clock();

        print_rom(&cpu.rom);
        print_ram(&cpu.ram);
        print_cpu_state(&cpu);

        thread::sleep(delay);
        i += 1;
    }

    Ok(())
}
// TODO: check correct functionallity