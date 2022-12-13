#[cfg(test)]

use intel4004_emu::intel4004::Intel4004;

#[test]
fn test_wrm() {
    let mut cpu = Intel4004::new();

    cpu.set_acc(0x7);
    cpu.set_ram_addrs(0xF);

    cpu.decode_op(0xE0);
    assert_eq!(cpu.ram.ram[0xF], 0x7);
}

#[test]
fn test_wmp() {
    let mut cpu = Intel4004::new();

    cpu.set_acc(0x7);

    cpu.decode_op(0xE1);
    assert_eq!(cpu.ram.output, 0x7);
}

#[test]
fn test_wrr() {
    let mut cpu = Intel4004::new();

    cpu.set_acc(0x7);

    cpu.decode_op(0xE2);
    assert_eq!(cpu.rom.io, 0x7);
}

#[test]
fn test_wr0() {
    let mut cpu = Intel4004::new();
    let ram_register = ((cpu.get_ram_addrs() & 0xF0) >> 4) & 0x03; 

    cpu.set_acc(0x7);

    cpu.decode_op(0xE4);
    assert_eq!(cpu.ram.status[(ram_register * 4) as usize], 0x7);
}

#[test]
fn test_wr1() {
    let mut cpu = Intel4004::new();
    let ram_register = ((cpu.get_ram_addrs() & 0xF0) >> 4) & 0x03; 

    cpu.set_acc(0x7);

    cpu.decode_op(0xE5);
    assert_eq!(cpu.ram.status[((ram_register * 4) + 1) as usize], 0x7);
}

#[test]
fn test_wr2() {
    let mut cpu = Intel4004::new();
    let ram_register = ((cpu.get_ram_addrs() & 0xF0) >> 4) & 0x03; 

    cpu.set_acc(0x7);

    cpu.decode_op(0xE6);
    assert_eq!(cpu.ram.status[((ram_register * 4) + 2) as usize], 0x7);
}

#[test]
fn test_wr3() {
    let mut cpu = Intel4004::new();
    let ram_register = ((cpu.get_ram_addrs() & 0xF0) >> 4) & 0x03; 

    cpu.set_acc(0x7);

    cpu.decode_op(0xE7);
    assert_eq!(cpu.ram.status[((ram_register * 4) + 3) as usize], 0x7);
}

#[test]
fn test_sbm() {
    let mut cpu = Intel4004::new();

    cpu.set_ram_addrs(0x1E);
    cpu.ram.ram[0x1E] = 0x4;

    cpu.set_acc(0x7);
    cpu.decode_op(0xE8);
    assert_eq!(cpu.get_acc(), 0x3);
}

#[test]
fn test_rdm() {
    let mut cpu = Intel4004::new();

    cpu.set_ram_addrs(0x1E);
    cpu.ram.ram[0x1E] = 0x4;

    cpu.set_acc(0x7);
    cpu.decode_op(0xE9);
    assert_eq!(cpu.get_acc(), 0x4);
}

#[test]
fn test_rdr() {
    let mut cpu = Intel4004::new();

    cpu.set_ram_addrs(0x1E);
    cpu.ram.ram[0x1E] = 0x4;

    cpu.set_acc(0x7);
    cpu.decode_op(0xEA);
    assert_eq!(cpu.get_acc(), 0x3);
}
