use crate::core::Core;

use super::*;

#[test]
fn test_ldx_immediate() {
    let bus = MockBus::new();
    let program = vec![0xa2, 0x69];
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.idx, 0x69);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 2);
}

#[test]
fn test_ldx_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0xa6, 0x20];
    bus.write(0x0020, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.idx, 0x69);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 3);
}

#[test]
fn test_ldx_zeropage_y() {
    let mut bus = MockBus::new();
    let program = vec![0xb6, 0x20];
    bus.write(0x0025, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x05;
    core.step();

    assert_eq!(core.idx, 0x69);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_ldx_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0xae, 0x37, 0x13];
    bus.write(0x1337, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.idx, 0x69);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_ldx_absolute_y() {
    let mut bus = MockBus::new();
    let program = vec![0xbe, 0x33, 0x13];
    bus.write(0x1337, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.step();

    assert_eq!(core.idx, 0x69);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_ldx_absolute_y_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xbe, 0xff, 0x20];
    bus.write(0x2100, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.step();

    assert_eq!(core.idx, 0x69);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 5);
}

#[test]
fn test_ldx_status_flags() {
    let bus = MockBus::new();
    let program = vec![0xa2, 0xff, 0xa2, 0x00];
    let mut core = Core::new(bus, program).unwrap();

    core.step();
    assert_eq!(core.status.as_byte(), 0b1000_0000);

    core.step();
    assert_eq!(core.status.as_byte(), 0b0000_0010);
}
