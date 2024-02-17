use crate::core::Core;

use super::*;

#[test]
fn sty_zeropage() {
    let bus = MockBus::new();
    let program = vec![0x84, 0x20];
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x69;
    core.step();
    let byte = core.get_bus().read(0x0020);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn sty_zeropage_x() {
    let bus = MockBus::new();
    let program = vec![0x94, 0x20];
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.idy = 0x69;
    core.step();
    let byte = core.get_bus().read(0x0022);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn sty_absolute() {
    let bus = MockBus::new();
    let program = vec![0x8C, 0x37, 0x13];
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x69;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 4));
}
