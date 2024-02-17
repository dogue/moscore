use crate::core::Core;

use super::*;

#[test]
fn inc_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0xe6, 0x20];
    bus.write(0x0020, 0x68);
    let mut core = Core::new(bus, program).unwrap();
    core.step();
    let byte = core.get_bus().read(0x0020);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn inc_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0xf6, 0x20];
    bus.write(0x0022, 0x68);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.step();
    let byte = core.get_bus().read(0x0022);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn inc_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0xee, 0x37, 0x13];
    bus.write(0x1337, 0x68);
    let mut core = Core::new(bus, program).unwrap();
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn inc_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0xfe, 0x33, 0x13];
    bus.write(0x1337, 0x68);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x04;
    core.step();
    let byte = core.get_bus().read(0x1337);

    assert_eq!(byte, 0x69);
    assert!(verify_clocks(&core, 7));
}

#[test]
fn inx() {
    let bus = MockBus::new();
    let program = vec![0xE8];
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x68;
    core.step();

    assert_eq!(core.idx, 0x69);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn iny() {
    let bus = MockBus::new();
    let program = vec![0xC8];
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x68;
    core.step();

    assert_eq!(core.idy, 0x69);
    assert!(verify_clocks(&core, 2));
}
