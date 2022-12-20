#[cfg(test)]

use intel4004_emu::intel4004::Intel4004;

use arbitrary_int::{u4};

#[test]
fn test_jcn() {
    let mut cpu = Intel4004::new();

    cpu.set_acc(0x7);
    cpu.set_ram_addrs(0xF);

    cpu.decode_op(0xE0);
    assert_eq!(cpu.ram.ram[0xF], 0x7);
}