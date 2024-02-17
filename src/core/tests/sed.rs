use crate::core::tests::verify_clocks;

use super::{Core, MockBus};

#[test]
fn sed() {
    let bus = MockBus::new();
    let program = vec![0xF8];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_decimal(false); // known initial state
    core.step();

    assert_eq!(core.status.decimal(), true);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn cld() {
    let bus = MockBus::new();
    let program = vec![0xD8];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_decimal(true);
    core.step();

    assert_eq!(core.status.decimal(), false);
    assert!(verify_clocks(&core, 2));
}
