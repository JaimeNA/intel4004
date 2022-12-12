#[cfg(test)]

use intel4004_emu::intel4004::Intel4004;

#[test]
fn test_clb() {
    let mut cpu = Intel4004::new();

    cpu.set_acc(0x7);
    cpu.set_carry(true);

    cpu.decode_op(0xF0);

    assert_eq!(cpu.get_acc(), 0x0);
    assert_eq!(cpu.get_carry(), false);
}

#[test]
fn test_clc() {
    let mut cpu = Intel4004::new();

    cpu.set_carry(true);

    cpu.decode_op(0xF1);
    assert_eq!(cpu.get_carry(), false);
}

#[test]
fn test_iac() {
    let mut cpu = Intel4004::new();

    cpu.set_acc(0x7);

    cpu.decode_op(0xF2);
    assert_eq!(cpu.get_acc(), 0x8);
}

#[test]
fn test_cmc() {
    let mut cpu = Intel4004::new();

    cpu.set_carry(false);

    cpu.decode_op(0xF3);
    assert_eq!(cpu.get_carry(), true);
}

#[test]
fn test_cma() {
    let mut cpu = Intel4004::new();

    cpu.set_acc(0x9);

    cpu.decode_op(0xF4);
    assert_eq!(cpu.get_acc() & 0x0F, 0x6);            // TODO: implement typoe u4 into accumulator.
}
