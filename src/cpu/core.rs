#![allow(arithmetic_overflow)]
use crate::{error::CoreError, traits::Bus};
use std::{cell::RefCell, cell::RefMut, rc::Rc};

use super::addressing::{Mode, Offset};

#[derive(Debug)]
pub struct Core {
    acc: u8,
    idx: u8,
    idy: u8,
    sp: u8,
    pc: u16,
    status: u8,
    bus: Rc<RefCell<dyn Bus>>,
    halted: bool,
}

impl Core {
    pub fn new<B: Bus>(bus: B, program: Vec<u8>) -> Result<Self, CoreError> {
        let mut core = Self {
            acc: 0,
            idx: 0,
            idy: 0,
            sp: 0,
            pc: 0,
            status: 0,
            bus: Rc::new(RefCell::new(bus)),
            halted: true,
        };

        core.bus.borrow_mut().load_rom(program)?;
        core.reset();

        Ok(core)
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.idx = 0;
        self.idy = 0;
        self.sp = 0xff;

        let mut bus = self.bus.borrow_mut();
        let low = bus.read(0xfffc);
        let high = bus.read(0xfffd);
        self.pc = self.addr_from_bytes(low, high);
        self.halted = false;
    }

    pub fn dump_status(&self) -> u8 {
        self.status
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let byte = self.fetch();
        self.decode(byte);
    }

    pub fn get_bus(&self) -> RefMut<dyn Bus> {
        self.bus.borrow_mut()
    }

    fn clock_bus(&self) {
        let mut bus = self.bus.borrow_mut();
        bus.on_clock();
    }

    fn read_bus(&self, addr: u16) -> u8 {
        let byte = self.bus.borrow_mut().read(addr);
        self.clock_bus();
        byte
    }

    fn write_bus(&mut self, addr: u16, byte: u8) {
        self.bus.borrow_mut().write(addr, byte);
        self.clock_bus();
    }

    fn fetch(&mut self) -> u8 {
        let byte = self.read_bus(self.pc);
        self.pc += 1;
        byte
    }

    fn addr_from_bytes(&self, low: u8, high: u8) -> u16 {
        (u16::from(high) << 8) | u16::from(low)
    }

    fn page_crossed(&self, byte: u8, index: u8) -> bool {
        (byte as u16) + (index as u16) > 255
    }

    fn set_carry(&mut self, cond: bool) {
        if cond {
            self.status |= 0x01;
        } else {
            self.status &= 0xfe;
        }
    }

    fn shift_byte(&mut self, byte: u8) -> u8 {
        let shifted = byte >> 1;
        self.set_carry(byte & 0x01 != 0);
        self.clock_bus();
        shifted
    }

    fn decode(&mut self, byte: u8) {
        match byte {
            0xA9 => self.load_a(Mode::Immediate),
            0xA5 => self.load_a(Mode::ZeroPage(Offset::None)),
            0xB5 => self.load_a(Mode::ZeroPage(Offset::X)),
            0xAD => self.load_a(Mode::Absolute(Offset::None)),
            0xBD => self.load_a(Mode::Absolute(Offset::X)),
            0xB9 => self.load_a(Mode::Absolute(Offset::Y)),
            0xA1 => self.load_a(Mode::IndexedIndirect),
            0xB1 => self.load_a(Mode::IndirectIndexed),
            0xA2 => self.load_x(Mode::Immediate),
            0xA6 => self.load_x(Mode::ZeroPage(Offset::None)),
            0xB6 => self.load_x(Mode::ZeroPage(Offset::Y)),
            0xAE => self.load_x(Mode::Absolute(Offset::None)),
            0xBE => self.load_x(Mode::Absolute(Offset::Y)),
            0xA0 => self.load_y(Mode::Immediate),
            0xA4 => self.load_y(Mode::ZeroPage(Offset::None)),
            0xB4 => self.load_y(Mode::ZeroPage(Offset::X)),
            0xAC => self.load_y(Mode::Absolute(Offset::None)),
            0xBC => self.load_y(Mode::Absolute(Offset::X)),
            0x4A => self.shift_right(Mode::Accumulator),
            0x46 => self.shift_right(Mode::ZeroPage(Offset::None)),
            0x56 => self.shift_right(Mode::ZeroPage(Offset::X)),
            0x4E => self.shift_right(Mode::Absolute(Offset::None)),
            0x5E => self.shift_right(Mode::Absolute(Offset::X)),
            0xEA => self.clock_bus(), // NOP
            _ => self.halted = true,
        }
    }
}

// addressing helpers
impl Core {
    /// returns (address: u16, page_crossed: bool)
    fn get_absolute(&mut self, offset: Offset) -> (u16, bool) {
        let mut page_crossed = false;
        match offset {
            Offset::None => {
                let low = self.fetch();
                let high = self.fetch();
                let addr = self.addr_from_bytes(low, high);
                (addr, page_crossed)
            }
            Offset::X => {
                let low = self.fetch();
                let high = self.fetch();
                let addr = self.addr_from_bytes(low, high);
                let addr = addr + self.idx as u16;
                if self.page_crossed(low, self.idx) {
                    self.clock_bus();
                    page_crossed = true;
                }
                (addr, page_crossed)
            }
            Offset::Y => {
                let low = self.fetch();
                let high = self.fetch();
                let addr = self.addr_from_bytes(low, high);
                let addr = addr + self.idy as u16;
                if self.page_crossed(low, self.idy) {
                    self.clock_bus();
                    page_crossed = true;
                }
                (addr, page_crossed)
            }
        }
    }

    fn get_zeropage(&mut self, offset: Offset) -> u16 {
        match offset {
            Offset::None => {
                let low = self.fetch();
                let addr = self.addr_from_bytes(low, 0x00);
                addr
            }
            Offset::X => {
                let low = self.fetch().wrapping_add(self.idx);
                self.clock_bus();
                let addr = self.addr_from_bytes(low, 0x00);
                addr
            }
            Offset::Y => {
                let low = self.fetch().wrapping_add(self.idy);
                self.clock_bus();
                let addr = self.addr_from_bytes(low, 0x00);
                addr
            }
        }
    }

    fn get_indexed_indirect(&mut self) -> u16 {
        let byte = self.fetch().wrapping_add(self.idx);
        let addr = self.addr_from_bytes(byte, 0x00);
        let low = self.read_bus(addr);
        let high = self.read_bus(addr + 1);
        self.clock_bus();
        let indirect = self.addr_from_bytes(low, high);
        indirect
    }

    fn get_indirect_indexed(&mut self) -> (u16, bool) {
        let mut page_crossed = false;
        let byte = self.fetch();
        let addr = self.addr_from_bytes(byte, 0x00);
        let low = self.read_bus(addr);
        let high = self.read_bus(addr + 1);
        if self.page_crossed(low, self.idy) {
            self.clock_bus();
            page_crossed = true;
        }
        let indirect = self.addr_from_bytes(low, high) + self.idy as u16;
        (indirect, page_crossed)
    }
}

// instructions
impl Core {
    fn load_a(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::Immediate => {
                self.acc = self.fetch();
                return;
            }
            Mode::Absolute(offset) => self.get_absolute(offset).0,
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            Mode::IndexedIndirect => self.get_indexed_indirect(),
            Mode::IndirectIndexed => self.get_indirect_indexed().0,
            _ => unimplemented!(),
        };

        self.acc = self.read_bus(addr);
    }

    fn load_x(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::Immediate => {
                self.idx = self.fetch();
                return;
            }
            Mode::Absolute(offset) => self.get_absolute(offset).0,
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            _ => unimplemented!(),
        };

        self.idy = self.read_bus(addr);
    }

    fn load_y(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::Immediate => {
                self.idy = self.fetch();
                return;
            }
            Mode::Absolute(offset) => self.get_absolute(offset).0,
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            _ => unimplemented!(),
        };

        self.idy = self.read_bus(addr);
    }

    fn shift_right(&mut self, mode: Mode) {
        match mode {
            Mode::Accumulator => {
                let byte = self.acc;
                self.acc = self.shift_byte(byte);
            }
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                let byte = self.read_bus(addr);
                let byte = self.shift_byte(byte);
                self.write_bus(addr, byte);
            }
            Mode::Absolute(offset) => {
                let (addr, crossed) = self.get_absolute(offset);
                // shitty little hack to keep the clock cycles correct
                if offset == Offset::X && !crossed {
                    self.clock_bus();
                }
                let byte = self.read_bus(addr);
                let byte = self.shift_byte(byte);
                self.write_bus(addr, byte);
            }
            _ => unimplemented!("invalid addressing mode for LSR"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::default::DefaultBus;

    use super::*;

    fn create_prog(byte: u8) -> Vec<u8> {
        let mut prog: [u8; 0x7fff] = [0; 0x7fff];
        prog[0x7ffd] = 0x80;
        prog[0x0000] = byte;

        prog.to_vec()
    }

    #[test]
    fn test_load_a_immediate() {
        let bus = DefaultBus::default();
        let mut prog = create_prog(0xa9);
        prog[0x0001] = 0x69;
        let mut core = Core::new(bus, prog).unwrap();
        core.run();

        assert_eq!(core.acc, 0x69);
    }

    #[test]
    fn test_load_a_zeropage() {
        let mut bus = DefaultBus::default();
        let mut prog = create_prog(0xa5);
        prog[0x0001] = 0x42;
        bus.write(0x0042, 0x69);
        let mut core = Core::new(bus, prog).unwrap();
        core.run();

        assert_eq!(core.acc, 0x69);
    }

    #[test]
    fn test_load_a_zeropage_x() {
        let mut bus = DefaultBus::default();
        let mut prog = create_prog(0xb5);
        prog[0x0001] = 0x40;
        bus.write(0x0042, 0x69);
        let mut core = Core::new(bus, prog.clone()).unwrap();
        core.idx = 0x02;
        core.run();

        assert_eq!(core.acc, 0x69);
    }

    #[test]
    fn test_load_a_zeropage_x_with_wrap() {
        let mut bus = DefaultBus::default();
        let mut prog = create_prog(0xb5);
        prog[0x0001] = 0xff;
        bus.write(0x0001, 0x69);
        let mut core = Core::new(bus, prog).unwrap();
        core.idx = 0x02;
        core.run();

        assert_eq!(core.acc, 0x69);
    }
}
