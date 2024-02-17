use crate::core::tests::verify_clocks;

use super::{Core, MockBus};

#[test]
fn sei() {
    let bus = MockBus::new();
    let program = vec![0x78];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_interrupt(false); // known initial state
    core.step();

    assert_eq!(core.status.interrupt(), true);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn cli() {
    let bus = MockBus::new();
    let program = vec![0x58];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_interrupt(true);
    core.step();

    assert_eq!(core.status.interrupt(), false);
    assert!(verify_clocks(&core, 2));
}
