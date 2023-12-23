use moscore::{core::Core, traits::Bus, *};

#[derive(Debug)]
struct TestBus {
    rom: [u8; 0xffff],
}

impl Bus for TestBus {
    fn read(&mut self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.rom[addr as usize] = byte;
    }

    fn on_clock(&mut self) {
        self.rom[0x0000] += 1;
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

fn create_prog(bytes: Vec<u8>) -> [u8; 0xffff] {
    let mut prog: [u8; 0xffff] = [0; 0xffff];
    prog[0xfffd] = 0x80;

    bytes
        .into_iter()
        .enumerate()
        .for_each(|(i, byte)| prog[i + 0x8000] = byte);

    prog
}

#[test]
fn count_load_a_immediate() {
    let bus = TestBus { rom: [0; 0xffff] };

    let prog = create_prog(vec![0xa9, 0x69]);

    let mut core = Core::new(bus, prog.to_vec()).unwrap();
    core.run();

    let rom = core.get_bus().dump_rom();
    assert_eq!(rom[0], 3);
}

#[test]
fn count_load_a_absolute() {
    let bus = TestBus { rom: [0; 0xffff] };
    let prog = create_prog(vec![0xad, 0xfd, 0xff]);
    let mut core = Core::new(bus, prog.to_vec()).unwrap();
    core.run();

    let rom = core.get_bus().dump_rom();
    assert_eq!(rom[0], 5);
}

#[test]
fn count_load_a_zeropage() {
    let bus = TestBus { rom: [0; 0xffff] };
    let prog = create_prog(vec![0xa5, 0x69]);
    let mut core = Core::new(bus, prog.to_vec()).unwrap();
    core.run();

    let rom = core.get_bus().dump_rom();
    assert_eq!(rom[0], 4);
}

#[test]
fn count_load_a_zeropage_x() {
    let bus = TestBus { rom: [0; 0xffff] };
    let prog = create_prog(vec![0xb5, 0x69]);
    let mut core = Core::new(bus, prog.to_vec()).unwrap();
    core.run();

    let rom = core.get_bus().dump_rom();
    assert_eq!(rom[0], 5);
}

#[test]
fn count_load_a_absolute_x() {
    let bus = TestBus { rom: [0; 0xffff] };
    let prog = create_prog(vec![0xbd, 0x42, 0x42]);
    let mut core = Core::new(bus, prog.to_vec()).unwrap();
    core.run();

    let rom = core.get_bus().dump_rom();
    assert_eq!(rom[0], 5);
}

#[test]
fn count_load_a_indirect_x() {
    let bus = TestBus { rom: [0; 0xffff] };
    let prog = create_prog(vec![0xa1, 0x69]);
    let mut core = Core::new(bus, prog.to_vec()).unwrap();
    core.run();

    let rom = core.get_bus().dump_rom();
    assert_eq!(rom[0], 7);
}

#[test]
fn count_load_a_indirect_y() {
    let bus = TestBus { rom: [0; 0xffff] };
    let prog = create_prog(vec![0xb1, 0x69]);
    let mut core = Core::new(bus, prog.to_vec()).unwrap();
    core.run();

    let rom = core.get_bus().dump_rom();
    assert_eq!(rom[0], 6);
}
