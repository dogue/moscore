use crate::core::Core;

use super::*;

#[test]
fn test_and_immediate() {
    let bus = MockBus::new();
    let program = vec![0x29, 0b0000_1000];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0001_1000;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 2);
}

#[test]
fn test_and_zeropage() {
    let mut bus = MockBus::new();
    let program = vec![0x25, 0x20];
    bus.write(0x0020, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0001_1000;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 3);
}

#[test]
fn test_and_zeropage_x() {
    let mut bus = MockBus::new();
    let program = vec![0x35, 0x20];
    bus.write(0x0025, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x05;
    core.acc = 0b0001_1000;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_and_absolute() {
    let mut bus = MockBus::new();
    let program = vec![0x2d, 0x37, 0x13];
    bus.write(0x1337, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0001_1000;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_and_absolute_x() {
    let mut bus = MockBus::new();
    let program = vec![0x3d, 0x33, 0x13];
    bus.write(0x1337, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0001_1000;
    core.idx = 0x04;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_and_aboslute_x_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0x3d, 0xff, 0x20];
    bus.write(0x2100, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x01;
    core.acc = 0b0001_1000;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 5);
}

#[test]
fn test_and_absolute_y() {
    let mut bus = MockBus::new();
    let program = vec![0x39, 0x33, 0x13];
    bus.write(0x1337, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0b0001_1000;
    core.idy = 0x04;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_and_absolute_y_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0x39, 0xff, 0x20];
    bus.write(0x2100, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.acc = 0b0001_1000;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 5);
}

#[test]
fn test_and_indexed_indirect() {
    let mut bus = MockBus::new();
    let program = vec![0x21, 0x20];
    bus.write(0x0022, 0x37);
    bus.write(0x0023, 0x13);
    bus.write(0x1337, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.idx = 0x02;
    core.acc = 0b0001_1000;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 6);
}

#[test]
fn test_and_indirect_indexed() {
    let mut bus = MockBus::new();
    let program = vec![0x31, 0x20];
    bus.write(0x0020, 0x33);
    bus.write(0x0021, 0x13);
    bus.write(0x1337, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x04;
    core.acc = 0b0001_1000;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 5);
}

#[test]
fn test_and_indirect_indexed_page_crossed() {
    let mut bus = MockBus::new();
    let program = vec![0x31, 0x20];
    bus.write(0x0020, 0xff);
    bus.write(0x0021, 0x30);
    bus.write(0x3100, 0b0000_1000);
    let mut core = Core::new(bus, program).unwrap();
    core.idy = 0x01;
    core.acc = 0b0001_1000;
    core.step();

    assert_eq!(core.acc, 0b0000_1000);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 6);
}
