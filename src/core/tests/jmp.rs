use crate::core::Core;

use super::*;

#[test]
fn jmp_absolute() {
    let bus = MockBus::new();
    let program = vec![0x4C, 0x20, 0x20];
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.pc, 0x2020);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn jmp_indirect() {
    let mut bus = MockBus::new();
    let program = vec![0x6C, 0x20, 0x20];
    bus.write(0x2020, 0x37);
    bus.write(0x2021, 0x13);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.pc, 0x1337);
    assert!(verify_clocks(&core, 5));
}
