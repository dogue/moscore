use moscore::{cpu::Core, traits::Bus, *};

#[derive(Debug)]
struct CycleCounterBus {
    rom: [u8; 0xffff],
}

impl Bus for CycleCounterBus {
    fn read(&mut self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.rom[addr as usize] = byte;
    }

    fn on_clock(&mut self) {
        self.rom[0x4242] += 1;
    }

    fn load_rom(&mut self, prog: Vec<u8>) -> Result<(), error::BusError> {
        prog.into_iter()
            .enumerate()
            .for_each(|(i, byte)| self.rom[i] = byte);
        Ok(())
    }

    fn dump_rom(&self) -> Vec<u8> {
        self.rom.to_vec()
    }
}

struct Test {
    opcode: u8,
    expected_cycles: u8,
}

impl Test {
    pub fn new(opcode: u8, expected_cycles: u8) -> Self {
        Self {
            opcode,
            expected_cycles,
        }
    }
}

fn create_prog(byte: u8) -> Vec<u8> {
    let mut prog: [u8; 0xffff] = [0; 0xffff];
    prog[0xfffd] = 0x80;
    prog[0x8000] = byte;

    prog.to_vec()
}

fn run_test(test: Test) {
    let bus = CycleCounterBus { rom: [0; 0xffff] };
    let prog = create_prog(test.opcode);
    let mut core = Core::new(bus, prog).unwrap();
    core.run();
    let rom = core.get_bus().dump_rom();
    // add one to account for fetching the invalid byte that halts
    assert_eq!(
        rom[0x4242],
        test.expected_cycles + 1,
        "failed on opcode: 0x{:0>2X}",
        test.opcode
    );
}

#[test]
fn test_lda_cycle_counts() {
    vec![
        Test::new(0xa9, 2),
        Test::new(0xa5, 3),
        Test::new(0xb5, 4),
        Test::new(0xad, 4),
        Test::new(0xbd, 4),
        Test::new(0xb9, 4),
        Test::new(0xa1, 6),
        Test::new(0xb1, 5),
    ]
    .into_iter()
    .for_each(|t| run_test(t));
}

#[test]
fn test_ldx_cycle_counts() {
    vec![
        Test::new(0xa2, 2),
        Test::new(0xa6, 3),
        Test::new(0xb6, 4),
        Test::new(0xae, 4),
        Test::new(0xbe, 4),
    ]
    .into_iter()
    .for_each(|t| run_test(t));
}

#[test]
fn test_ldy_cycle_counts() {
    vec![
        Test::new(0xa0, 2),
        Test::new(0xa4, 3),
        Test::new(0xb4, 4),
        Test::new(0xac, 4),
        Test::new(0xbc, 4),
    ]
    .into_iter()
    .for_each(|t| run_test(t));
}

#[test]
fn test_lsr_cycle_counts() {
    vec![
        Test::new(0x4a, 2),
        Test::new(0x46, 5),
        Test::new(0x56, 6),
        Test::new(0x4e, 6),
        Test::new(0x5e, 7),
    ]
    .into_iter()
    .for_each(|t| run_test(t));
}
