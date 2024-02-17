use crate::core::Core;

use super::*;

#[test]
fn tsx() {
    let bus = MockBus::new();
    let program = vec![0xBA];
    let mut core = Core::new(bus, program).unwrap();
    core.sp = 0x69;
    core.step();

    assert_eq!(core.idx, 0x69);
    assert!(verify_clocks(&core, 2));
}
