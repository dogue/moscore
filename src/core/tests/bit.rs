use crate::core::Core;

use super::*;

#[test]
fn bit_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0x24, 0x69];
    bus.write(0x0069, 0b1001_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0000_1000;
    core.step();

    assert_eq!(core.status.zero(), false);
    assert_eq!(core.status.overflow(), false);
    assert_eq!(core.status.negative(), true);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn bit_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0x2C, 0x20, 0x20];
    bus.write(0x2020, 0b0100_0000);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0000_1000;
    core.step();

    assert_eq!(core.status.zero(), true);
    assert_eq!(core.status.overflow(), true);
    assert_eq!(core.status.negative(), false);
    assert!(verify_clocks(&core, 4));
}
