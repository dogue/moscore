use crate::core::Core;

use super::*;

#[test]
fn test_adc_immediate() {
    let bus = MockBus::new();
    let program = vec![0x69, 0x08];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 2);
}

#[test]
fn test_adc_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0x65, 0x20];
    bus.write(0x0020, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 3);
}

#[test]
fn test_adc_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0x75, 0x20];
    bus.write(0x0025, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_adc_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0x6d, 0x37, 0x13];
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_adc_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0x7d, 0x33, 0x13];
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x04;
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_adc_absolute_x_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0x7d, 0xff, 0x20];
    bus.write(0x2100, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x01;
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 5);
}

#[test]
fn test_adc_absolute_y() {
    let mut bus = MockBus::new();
    let program = vec![0x79, 0x33, 0x13];
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_adc_absolute_y_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0x79, 0xff, 0x20];
    bus.write(0x2100, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 5);
}

#[test]
fn test_adc_indexed_indirect() {
    let mut bus = MockBus::new();
    let program = vec![0x61, 0x20];
    bus.write(0x0022, 0x37);
    bus.write(0x0023, 0x13);
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 6);
}

#[test]
fn test_adc_indirect_indexed() {
    let mut bus = MockBus::new();
    let program = vec![0x71, 0x20];
    bus.write(0x0020, 0x33);
    bus.write(0x0021, 0x13);
    bus.write(0x1337, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 5);
}

#[test]
fn test_adc_indirect_indexed_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0x71, 0x20];
    bus.write(0x0020, 0xff);
    bus.write(0x0021, 0x20);
    bus.write(0x2100, 0x08);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.acc = 0x08;
    core.step();

    assert_eq!(core.acc, 0x10);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 6);
}

#[test]
fn test_adc_status_flags() {
    let bus = MockBus::new();
    let program = vec![0x69, 0xfe, 0x69, 0x01, 0x69, 0x50, 0x69, 0x80];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x1;

    core.step();
    assert_eq!(core.status.as_byte(), 0b1000_0000);

    core.step();
    assert_eq!(core.status.as_byte(), 0b0000_0011);

    core.acc = 0x50;
    core.step();
    assert_eq!(core.status.as_byte(), 0b1100_0000);

    core.acc = 0x80;
    core.step();
    assert_eq!(core.status.as_byte(), 0b0100_0011);
}
