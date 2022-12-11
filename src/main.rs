use std::io;

use intel4004_emu::intel4004::Intel4004;
use intel4004_emu::disassembler::*;

fn main() -> io::Result<()>{

    let mut cpu = Intel4004::new();

    cpu.rom.load_rom("rom/WRM")?;

    let mut i = 0;

    while i < 1000 {
        cpu.clock();
        i += 1;
    }

    print_rom(&cpu.rom);
    print_ram(&cpu.ram);
    print_cpu_state(&cpu);

    Ok(())
}
