use crate::core::Core;

use super::*;

#[test]
fn ror_acc() {
    let bus = MockBus::new();
    let program = vec![0x6a];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0110_0110;
    core.status.set_carry(true);
    core.step();

    assert_eq!(core.acc, 0b1011_0011, "got {:0>8b}", core.acc);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn ror_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0x66, 0x20];
    bus.write(0x0020, 0b0110_0110);
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_carry(true);
    core.step();
    let byte = core.get_bus().read(0x0020);

    assert_eq!(byte, 0b1011_0011);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn ror_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0x76, 0x20];
    bus.write(0x0025, 0b0110_0110);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.status.set_carry(true);
    core.step();
    let byte = core.get_bus().read(0x0025);

    assert_eq!(byte, 0b1011_0011);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn ror_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0x6e, 0x37, 0x13];
    bus.write(0x1337, 0b0110_0110);
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_carry(true);
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0b1011_0011);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn ror_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0x7e, 0x33, 0x13];
    bus.write(0x1337, 0b0110_0110);
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_carry(true);
    core.idx = 0x04;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0b1011_0011);
    assert!(verify_clocks(&core, 7));
}
