use std::io;

use intel4004_emu::intel4004::Intel4004;
use intel4004_emu::disassembler::*;

fn main() -> io::Result<()>{

    let mut cpu = Intel4004::new();

    cpu.rom.load_rom("rom/RDn")?;

    cpu.clock();
    cpu.clock();
    cpu.clock();

    print_rom(&cpu.rom);
    print_cpu_state(&cpu);

    Ok(())
}
