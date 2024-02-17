use crate::core::Core;

use super::*;

#[test]
fn lsr_acc() {
    let bus = MockBus::new();
    let program = vec![0x4a];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b1000_1000;
    core.step();

    assert_eq!(core.acc, 0b0100_0100);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn lsr_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0x46, 0x20];
    bus.write(0x0020, 0b1000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.step();
    let byte = core.get_bus().read(0x0020);

    assert_eq!(byte, 0b0100_0100);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn lsr_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0x56, 0x20];
    bus.write(0x0025, 0b1000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.step();
    let byte = core.get_bus().read(0x0025);

    assert_eq!(byte, 0b0100_0100);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn lsr_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0x4e, 0x37, 0x13];
    bus.write(0x1337, 0b1000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0b0100_0100);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn lsr_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0x5e, 0x33, 0x13];
    bus.write(0x1337, 0b1000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x04;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0b0100_0100);
    assert!(verify_clocks(&core, 7));
}

#[test]
fn lsr_status_flags() {
    let bus = MockBus::new();
    let program = vec![0x4a, 0x4a];
    let mut core = Core::new(bus, program).unwrap();

    core.acc = 0b0000_0010;
    core.step();
    assert_eq!(core.status.as_byte(), 0b0000_0000);

    core.step();
    assert_eq!(core.status.as_byte(), 0b0000_0011);
}
