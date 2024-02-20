use crate::core::Core;

use super::*;

#[test]
fn jsr() {
    let bus = MockBus::new();
    let program = vec![0x20, 0x37, 0x13];
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.pc, 0x1337);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn rts() {
    let bus = MockBus::new();
    let program = vec![0x60];
    let mut core = Core::new(bus, program).unwrap();
    core.sp -= 2;
    core.get_bus().write(0x01FE, 0x13);
    core.get_bus().write(0x01FD, 0x37);
    core.step();

    assert!(core.pc == 0x1337);
    assert!(verify_clocks(&core, 6));
}
