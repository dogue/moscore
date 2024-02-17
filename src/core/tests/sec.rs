use crate::core::tests::verify_clocks;

use super::{Core, MockBus};

#[test]
fn sec() {
    let bus = MockBus::new();
    let program = vec![0x38];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_carry(false); // known initial state
    core.step();

    assert_eq!(core.status.carry(), true);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn clc() {
    let bus = MockBus::new();
    let program = vec![0x18];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_carry(true);
    core.step();

    assert_eq!(core.status.carry(), false);
    assert!(verify_clocks(&core, 2));
}
