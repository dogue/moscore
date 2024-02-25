use super::*;

#[test]
fn bvc_false() {
    let bus = MockBus::new();
    let program = vec![0x50, 0x05];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_overflow(true);
    core.step();

    assert_eq!(core.pc, 0x02);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn bvc_true() {
    let bus = MockBus::new();
    let program = vec![0x50, 0x05];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_overflow(false);
    core.step();

    assert_eq!(core.pc, 0x07);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn bvc_page_crossed() {
    let mut bus = MockBus::new();
    bus.write(0x80F0, 0x50);
    bus.write(0x80F1, 0x0F);
    bus.write(0xFFFC, 0xF0);
    bus.write(0xFFFD, 0x80);
    let mut core = Core::new(bus, vec![]).unwrap();
    core.status.set_overflow(false);
    core.step();

    assert_eq!(core.pc, 0x8101);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn bvc_negative_offset() {
    let mut bus = MockBus::new();
    bus.write(0x8020, 0x50);
    bus.write(0x8021, 0x81);
    bus.write(0xFFFC, 0x20);
    bus.write(0xFFFD, 0x80);
    let mut core = Core::new(bus, vec![]).unwrap();
    core.status.set_overflow(false);
    core.step();

    assert_eq!(core.pc, 0x8021);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn bvc_negative_offset_page_crossed() {
    let mut bus = MockBus::new();
    bus.write(0x8000, 0x50);
    bus.write(0x8001, 0x83);
    bus.write(0xFFFC, 0x00);
    bus.write(0xFFFD, 0x80);
    let mut core = Core::new(bus, vec![]).unwrap();
    core.status.set_overflow(false);
    core.step();

    assert_eq!(core.pc, 0x7FFF);
    assert!(verify_clocks(&core, 4));
}
