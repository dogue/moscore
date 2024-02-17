use crate::core::{
    tests::{verify_clocks, MockBus},
    Core,
};

#[test]
fn clv() {
    let bus = MockBus::new();
    let program = vec![0xB8];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_overflow(true);
    core.step();

    assert_eq!(core.status.overflow(), false);
    assert!(verify_clocks(&core, 2));
}
