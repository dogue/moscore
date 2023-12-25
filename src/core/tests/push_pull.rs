use crate::core::Core;

use super::*;

#[test]
fn test_pha() {
    let bus = MockBus::new();
    let program = vec![0x48];
    let mut core = Core::new(bus, program).unwrap();
    core.acc = 0x69;
    core.step();
    let byte = core.get_bus().read(0x01fe);

    assert_eq!(byte, 0x69);
    assert_eq!(core.sp, 0xfe);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 3);
}

#[test]
fn test_pla() {
    let mut bus = MockBus::new();
    let program = vec![0x68];
    bus.write(0x01ff, 0x69);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.acc, 0x69);
    assert_eq!(core.sp, 0x00);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}

#[test]
fn test_php() {
    let bus = MockBus::new();
    let program = vec![0x08];
    let mut core = Core::new(bus, program).unwrap();
    core.status.set_carry(true);
    core.status.set_negative(true);
    core.step();
    let byte = core.get_bus().read(0x01fe);

    assert_eq!(byte, 0x81);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 3);
}

#[test]
fn test_plp() {
    let mut bus = MockBus::new();
    let program = vec![0x28];
    bus.write(0x01ff, 0x81);
    let mut core = Core::new(bus, program).unwrap();
    core.step();

    assert_eq!(core.status.as_byte(), 0x81);

    let clocks = core.get_bus().read(0xc10c);
    assert_eq!(clocks, 4);
}
