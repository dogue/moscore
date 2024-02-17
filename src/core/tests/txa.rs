use crate::core::Core;

use super::*;

#[test]
fn txa() {
    let bus = MockBus::new();
    let program = vec![0x8A];
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x69;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 2));
}
