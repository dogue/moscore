use crate::core::Core;

use super::*;

#[test]
fn sta_zeropage() {
    let bus = MockBus::new();
    let program = vec![0x85, 0x20];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x69;
    core.step();
    let byte = core.get_bus().read(0x0020);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn sta_zeropage_x() {
    let bus = MockBus::new();
    let program = vec![0x95, 0x20];
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.acc = 0x69;
    core.step();
    let byte = core.get_bus().read(0x0022);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn sta_absolute() {
    let bus = MockBus::new();
    let program = vec![0x8D, 0x37, 0x13];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x69;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn sta_absolute_x() {
    let bus = MockBus::new();
    let program = vec![0x9D, 0x33, 0x13];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x69;
    core.idx = 0x04;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn sta_absolute_y() {
    let bus = MockBus::new();
    let program = vec![0x99, 0x33, 0x13];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x69;
    core.idy = 0x04;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn sta_indexed_indirect() {
    let mut bus = MockBus::new();
    let program = vec![0x81, 0x40];
    bus.write(0x0042, 0x37);
    bus.write(0x0043, 0x13);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.acc = 0x69;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn sta_indirect_indexed() {
    let mut bus = MockBus::new();
    let program = vec![0x91, 0x40];
    bus.write(0x0040, 0x33);
    bus.write(0x0041, 0x13);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x69;
    core.idy = 0x04;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 6));
}
