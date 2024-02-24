use super::*;

#[test]
fn lda_immediate() {
    let bus = MockBus::new();
    let program = vec![0xa9, 0x05];
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.acc, 0x05);
    assert!(verify_clocks(&core, 2));
}

#[test]
fn lda_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0xa5, 0x20];
    bus.write(0x0020, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 3));
}

#[test]
fn lda_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0xb5, 0x20];
    bus.write(0x0025, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn lda_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0xad, 0x37, 0x13];
    bus.write(0x1337, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn lda_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0xbd, 0x33, 0x13];
    bus.write(0x1337, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x04;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn lda_absolute_x_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xbd, 0xff, 0x20];
    bus.write(0x2100, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x01;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn lda_absolute_y() {
    let mut bus = MockBus::new();
    let program = vec![0xb9, 0x33, 0x13];
    bus.write(0x1337, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 4));
}

#[test]
fn lda_absolute_y_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xb9, 0xff, 0x20];
    bus.write(0x2100, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn lda_indexed_indirect() {
    let mut bus = MockBus::new();
    let program = vec![0xa1, 0x40];
    bus.write(0x0042, 0x37);
    bus.write(0x0043, 0x13);
    bus.write(0x1337, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn lda_indirect_indexed() {
    let mut bus = MockBus::new();
    let program = vec![0xb1, 0x40];
    bus.write(0x0040, 0x33);
    bus.write(0x0041, 0x13);
    bus.write(0x1337, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 5));
}

#[test]
fn lda_indirect_indexed_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0xb1, 0x40];
    bus.write(0x0040, 0xff);
    bus.write(0x0041, 0x20);
    bus.write(0x2100, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.step();

    assert_eq!(core.acc, 0x69);
    assert!(verify_clocks(&core, 6));
}

#[test]
fn lda_status_flags() {
    let bus = MockBus::new();
    let program = vec![0xa9, 0xff, 0xa9, 0x00];
    let mut core = Core::new(bus, program).unwrap();

    core.step();
    assert_eq!(core.status.as_byte(), 0b1000_0000);

    core.step();
    assert_eq!(core.status.as_byte(), 0b0000_0010);
}
