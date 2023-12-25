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
