use crate::core::Core;

use super::*;

#[test]
fn cpx_immediate() {
    let bus = MockBus::new();
    let program = vec![0xE0, 0x05];
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x0A;
    core.step();

    assert!(core.status.carry());
    assert!(!core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 2));
}

#[test]
fn cpx_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0xE4, 0x69];
    bus.write(0x0069, 0x05);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.step();

    assert!(core.status.carry());
    assert!(core.status.zero());
    assert!(!core.status.negative());
    assert!(verify_clocks(&core, 3));
}

#[test]
fn cpx_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0xEC, 0x37, 0x13];
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.step();

    assert!(!core.status.carry());
    assert!(!core.status.zero());
    assert!(core.status.negative());
    assert!(verify_clocks(&core, 4));
}
