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
