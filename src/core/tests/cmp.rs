use crate::core::Core;

use super::*;

#[test]
fn cmp_immediate() {
    let bus = MockBus::new();
    let program = vec![0xC9, 0x05];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 2));
}

#[test]
fn cmp_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0xC5, 0x69];
    bus.write(0x0069, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x05;
    core.step();

    assert!(core.status.carry());
    assert!(core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 3));
}

#[test]
fn cmp_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0xD5, 0x68];
    bus.write(0x0069, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x01;
    core.acc = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 4));
}

#[test]
fn cmp_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0xCD, 0x37, 0x13];
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x05;
    core.step();

    assert!(!core.status.carry());
    assert!(!core.status.zero());
    assert!(core.status.negative());
    assert!(verify_clocks(&core, 4));
}

#[test]
fn cmp_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0xDD, 0x33, 0x13];
    bus.write(0x1337, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x04;
    core.acc = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 4));
}

#[test]
fn cmp_absolute_x_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xDD, 0xFF, 0x20];
    bus.write(0x2100, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x01;
    core.acc = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 5));
}

#[test]
fn cmp_absolute_y() {
    let mut bus = MockBus::new();
    let program = vec![0xD9, 0x33, 0x13];
    bus.write(0x1337, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.acc = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 4));
}

#[test]
fn cmp_absolute_y_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xD9, 0xFF, 0x20];
    bus.write(0x2100, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.acc = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 5));
}

#[test]
fn cmp_indexed_indirect() {
    let mut bus = MockBus::new();
    let program = vec![0xC1, 0x40];
    bus.write(0x0042, 0x37);
    bus.write(0x0043, 0x13);
    bus.write(0x1337, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.acc = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 6));
}

#[test]
fn cmp_indirect_indexed() {
    let mut bus = MockBus::new();
    let program = vec![0xD1, 0x40];
    bus.write(0x0040, 0x33);
    bus.write(0x0041, 0x13);
    bus.write(0x1337, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.acc = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 5));
}

#[test]
fn cmp_indirect_indexed_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xD1, 0x40];
    bus.write(0x0040, 0xFF);
    bus.write(0x0041, 0x20);
    bus.write(0x2100, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.acc = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 6));
}
