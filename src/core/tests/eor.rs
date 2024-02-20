use crate::core::Core;

use super::*;

#[test]
fn eor_immediate() {
    let bus = MockBus::new();
    let program = vec![0x49, 0b0000_1000];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0000_1000;
    core.step();

    assert!(core.status.zero());
    assert!(!core.status.negative());
    assert!(core.acc == 0);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn eor_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0x45, 0x69];
    bus.write(0x0069, 0b1000_0000);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert!(core.status.negative());
    assert!(!core.status.zero());
    assert!(core.acc == 0x80);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn eor_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0x55, 0x20];
    bus.write(0x0022, 0b0000_0001);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.step();

    assert!(!core.status.negative());
    assert!(!core.status.zero());
    assert!(core.acc == 1);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn eor_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0x4D, 0x00, 0x20];
    bus.write(0x2000, 1);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert!(!core.status.negative());
    assert!(!core.status.zero());
    assert!(core.acc == 1);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn eor_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0x5D, 0x00, 0x20];
    bus.write(0x2020, 1);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x20;
    core.step();

    assert!(!core.status.negative());
    assert!(!core.status.zero());
    assert!(core.acc == 1);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn eor_absolute_x_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0x5D, 0xFF, 0x20];
    bus.write(0x2100, 1);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 1;
    core.step();

    assert!(!core.status.negative());
    assert!(!core.status.zero());
    assert!(core.acc == 1);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn eor_absolute_y() {
    let mut bus = MockBus::new();
    let program = vec![0x59, 0x00, 0x20];
    bus.write(0x2001, 1);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 1;
    core.step();

    assert!(core.acc == 1);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn eor_absolute_y_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0x59, 0xFF, 0x20];
    bus.write(0x2100, 1);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 1;
    core.step();

    assert!(core.acc == 1);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn eor_indexed_indirect() {
    let mut bus = MockBus::new();
    let program = vec![0x41, 0x40];
    bus.write(0x0042, 0x37);
    bus.write(0x0043, 0x13);
    bus.write(0x1337, 1);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 2;
    core.step();

    assert!(core.acc == 1);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn eor_indirect_indexed() {
    let mut bus = MockBus::new();
    let program = vec![0x51, 0x20];
    bus.write(0x0020, 0x33);
    bus.write(0x0021, 0x13);
    bus.write(0x1337, 1);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.step();

    assert!(core.acc == 1);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn eor_indirect_indexed_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0x51, 0x20];
    bus.write(0x0020, 0xFC);
    bus.write(0x0021, 0x13);
    bus.write(0x1400, 1);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.step();

    assert!(core.acc == 1);
    assert!(verify_clocks(&core, 6));
}
