use crate::core::Core;

use super::*;

#[test]
fn sbc_immediate() {
    let bus = MockBus::new();
    let program = vec![0xE9, 0x08];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn sbc_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0xE5, 0x20];
    bus.write(0x0020, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn sbc_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0xF5, 0x20];
    bus.write(0x0025, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn sbc_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0xED, 0x37, 0x13];
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn sbc_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0xFD, 0x33, 0x13];
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x04;
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn sbc_absolute_x_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xFD, 0xFF, 0x20];
    bus.write(0x2100, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x01;
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn sbc_absolute_y() {
    let mut bus = MockBus::new();
    let program = vec![0xF9, 0x33, 0x13];
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn sbc_absolute_y_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xF9, 0xFF, 0x20];
    bus.write(0x2100, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn sbc_indexed_indirect() {
    let mut bus = MockBus::new();
    let program = vec![0xE1, 0x20];
    bus.write(0x0022, 0x37);
    bus.write(0x0023, 0x13);
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn sbc_indirect_indexed() {
    let mut bus = MockBus::new();
    let program = vec![0xF1, 0x20];
    bus.write(0x0020, 0x33);
    bus.write(0x0021, 0x13);
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn sbc_indirect_indexed_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xF1, 0x20];
    bus.write(0x0020, 0xff);
    bus.write(0x0021, 0x20);
    bus.write(0x2100, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.acc = 0x10;
    core.step();

    assert_eq!(core.acc, 0x08);
    assert!(verify_clocks(&core, 6));
}
