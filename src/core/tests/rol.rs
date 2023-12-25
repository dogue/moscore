use crate::core::Core;

use super::*;

#[test]
fn test_rol_acc() {
    let bus = MockBus::new();
    let program = vec![0x2a];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0110_0110;
    core.status.set_carry(true);
    core.step();

    assert_eq!(core.acc, 0b1100_1101);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 2);

    core.reset();
    core.acc = 0b0110_0110;
    core.step();
    assert_eq!(core.acc, 0b1100_1100);
}

#[test]
fn test_rol_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0x26, 0x20];
    bus.write(0x0020, 0b0110_0110);
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_carry(true);
    core.step();
    let byte = core.get_bus().read(0x0020);

    assert_eq!(byte, 0b1100_1101);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 5);
}

#[test]
fn test_rol_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0x36, 0x20];
    bus.write(0x0025, 0b0110_0110);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.status.set_carry(true);
    core.step();
    let byte = core.get_bus().read(0x0025);

    assert_eq!(byte, 0b1100_1101);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 6);
}

#[test]
fn test_rol_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0x2e, 0x37, 0x13];
    bus.write(0x1337, 0b0110_0110);
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_carry(true);
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0b1100_1101);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 6);
}

#[test]
fn test_rol_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0x3e, 0x33, 0x13];
    bus.write(0x1337, 0b0110_0110);
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_carry(true);
    core.idx = 0x04;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0b1100_1101);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 7);
}
