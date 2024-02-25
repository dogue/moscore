use super::*;

#[test]
fn brk() {
    let mut bus = MockBus::new();
    let program = vec![0x00];
    bus.write(0xFFFE, 0x69);
    bus.write(0xFFFF, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.pc, 0x6969);
    assert!(verify_clocks(&core, 7));
}
