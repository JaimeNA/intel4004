#[cfg(test)]

use intel4004_emu::intel4004::Intel4004;

use arbitrary_int::{u4};

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
    assert_eq!(cpu.rom.io.value(), 0x7);
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

    cpu.rom.io = u4::new(0x3);

    cpu.set_acc(0x7);
    cpu.decode_op(0xEA);
    assert_eq!(cpu.get_acc(), 0x3);
}

#[test]
fn test_adm() {
    let mut cpu = Intel4004::new();

    cpu.set_ram_addrs(0x1E);
    cpu.ram.ram[0x1E] = 0x4;

    cpu.set_acc(0x7);
    cpu.decode_op(0xEB);
    assert_eq!(cpu.get_acc(), 0xB);
}

#[test]
fn test_rd0() {
    let mut cpu = Intel4004::new();
    let ram_register = ((cpu.get_ram_addrs() & 0xF0) >> 4) & 0x03; 

    cpu.ram.status[(ram_register * 4) as usize] = 0x7;

    cpu.decode_op(0xEC);
    assert_eq!(cpu.get_acc(), 0x7);
}

#[test]
fn test_rd1() {
    let mut cpu = Intel4004::new();
    let ram_register = ((cpu.get_ram_addrs() & 0xF0) >> 4) & 0x03; 

    cpu.ram.status[((ram_register * 4) + 1) as usize] = 0x7;

    cpu.decode_op(0xED);
    assert_eq!(cpu.get_acc(), 0x7);
}

#[test]
fn test_rd2() {
    let mut cpu = Intel4004::new();
    let ram_register = ((cpu.get_ram_addrs() & 0xF0) >> 4) & 0x03; 

    cpu.ram.status[((ram_register * 4) + 2) as usize] = 0x5;

    cpu.decode_op(0xEE);
    assert_eq!(cpu.get_acc(), 0x5);
}

#[test]
fn test_rd3() {
    let mut cpu = Intel4004::new();
    let ram_register = ((cpu.get_ram_addrs() & 0xF0) >> 4) & 0x03; 

    cpu.ram.status[((ram_register * 4) + 3) as usize] = 0x6;

    cpu.decode_op(0xEF);
    assert_eq!(cpu.get_acc(), 0x6);
}