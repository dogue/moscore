use crate::core::Core;

use super::*;

#[test]
fn tya() {
    let bus = MockBus::new();
    let program = vec![0x98];
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x69;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 2));
}
