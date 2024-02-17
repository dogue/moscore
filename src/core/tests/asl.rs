use crate::core::Core;

use super::*;

#[test]
fn lsr_acc() {
    let bus = MockBus::new();
    let program = vec![0x0A];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0100_0100;
    core.step();

    assert_eq!(core.acc, 0b1000_1000);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn lsr_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0x06, 0x20];
    bus.write(0x0020, 0b0100_0100);
    let mut core = Core::new(bus, program).unwrap();
    core.step();
    let byte = core.get_bus().read(0x0020);

    assert_eq!(byte, 0b1000_1000);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn lsr_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0x16, 0x20];
    bus.write(0x0025, 0b0100_0100);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.step();
    let byte = core.get_bus().read(0x0025);

    assert_eq!(byte, 0b1000_1000);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn lsr_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0x0E, 0x37, 0x13];
    bus.write(0x1337, 0b0100_0100);
    let mut core = Core::new(bus, program).unwrap();
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0b1000_1000);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn lsr_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0x1E, 0x33, 0x13];
    bus.write(0x1337, 0b0100_0100);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x04;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0b1000_1000);
    assert!(verify_clocks(&core, 7));
}
