use crate::core::Core;

use super::*;

#[test]
fn tay() {
    let bus = MockBus::new();
    let program = vec![0xA8];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x69;
    core.step();

    assert_eq!(core.idy, 0x69);
    assert!(verify_clocks(&core, 2));
}
