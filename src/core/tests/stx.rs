use crate::core::Core;

use super::*;

#[test]
fn stx_zeropage() {
    let bus = MockBus::new();
    let program = vec![0x86, 0x20];
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x69;
    core.step();
    let byte = core.get_bus().read(0x0020);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn stx_zeropage_y() {
    let bus = MockBus::new();
    let program = vec![0x96, 0x20];
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x02;
    core.idx = 0x69;
    core.step();
    let byte = core.get_bus().read(0x0022);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn stx_absolute() {
    let bus = MockBus::new();
    let program = vec![0x8E, 0x37, 0x13];
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x69;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 4));
}
