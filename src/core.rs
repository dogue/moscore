#![allow(arithmetic_overflow)]
use crate::{error::CoreError, traits::Bus};
use std::{cell::RefCell, cell::RefMut, rc::Rc};

use self::{
    addressing::{Mode, Offset},
    flags::Flags,
};
mod addressing;
mod flags;

#[derive(Debug)]
pub struct Core {
    acc: u8,
    idx: u8,
    idy: u8,
    sp: u8,
    pc: u16,
    status: Flags,
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
            status: Flags::new(),
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
        self.status.as_byte()
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

    fn shift_byte(&mut self, byte: u8) -> u8 {
        let shifted = byte >> 1;
        if byte & 0x01 != 0 {
            self.status.set_carry(true);
        } else {
            self.status.set_carry(false);
        }
        self.clock_bus();
        shifted
    }

    fn decode(&mut self, byte: u8) {
        match byte {
            0xA9 => self.lda(Mode::Immediate),
            0xA5 => self.lda(Mode::ZeroPage(Offset::None)),
            0xB5 => self.lda(Mode::ZeroPage(Offset::X)),
            0xAD => self.lda(Mode::Absolute(Offset::None)),
            0xBD => self.lda(Mode::Absolute(Offset::X)),
            0xB9 => self.lda(Mode::Absolute(Offset::Y)),
            0xA1 => self.lda(Mode::IndexedIndirect),
            0xB1 => self.lda(Mode::IndirectIndexed),
            0xA2 => self.ldx(Mode::Immediate),
            0xA6 => self.ldx(Mode::ZeroPage(Offset::None)),
            0xB6 => self.ldx(Mode::ZeroPage(Offset::Y)),
            0xAE => self.ldx(Mode::Absolute(Offset::None)),
            0xBE => self.ldx(Mode::Absolute(Offset::Y)),
            0xA0 => self.ldy(Mode::Immediate),
            0xA4 => self.ldy(Mode::ZeroPage(Offset::None)),
            0xB4 => self.ldy(Mode::ZeroPage(Offset::X)),
            0xAC => self.ldy(Mode::Absolute(Offset::None)),
            0xBC => self.ldy(Mode::Absolute(Offset::X)),
            0x4A => self.lsr(Mode::Accumulator),
            0x46 => self.lsr(Mode::ZeroPage(Offset::None)),
            0x56 => self.lsr(Mode::ZeroPage(Offset::X)),
            0x4E => self.lsr(Mode::Absolute(Offset::None)),
            0x5E => self.lsr(Mode::Absolute(Offset::X)),
            0x69 => self.adc(Mode::Immediate),
            0x65 => self.adc(Mode::ZeroPage(Offset::None)),
            0x75 => self.adc(Mode::ZeroPage(Offset::X)),
            0x6D => self.adc(Mode::Absolute(Offset::None)),
            0x7D => self.adc(Mode::Absolute(Offset::X)),
            0x79 => self.adc(Mode::Absolute(Offset::Y)),
            0x61 => self.adc(Mode::IndexedIndirect),
            0x71 => self.adc(Mode::IndirectIndexed),
            0xEA => self.clock_bus(), // NOP
            _ => self.halted = true,
        }
    }
}

// status flags
impl Core {
    fn set_nz(&mut self, byte: u8) {
        self.set_zero(byte).set_negative(byte);
    }

    fn set_zero(&mut self, byte: u8) -> &mut Self {
        if byte == 0 {
            self.status.set_zero(true);
        } else {
            self.status.set_zero(false);
        }
        self
    }

    fn set_negative(&mut self, byte: u8) -> &mut Self {
        if byte & 0b1000_0000 != 0 {
            self.status.set_negative(true);
        } else {
            self.status.set_negative(false);
        }
        self
    }

    fn check_overflow(&self, original: u8, value: u8, result: u8) -> bool {
        ((original ^ value) & 0x80 == 0) && ((original ^ result) & 0x80 != 0)
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
    fn lda(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::Immediate => {
                self.acc = self.fetch();
                self.set_nz(self.acc);
                return;
            }
            Mode::Absolute(offset) => self.get_absolute(offset).0,
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            Mode::IndexedIndirect => self.get_indexed_indirect(),
            Mode::IndirectIndexed => self.get_indirect_indexed().0,
            _ => unimplemented!(),
        };

        self.acc = self.read_bus(addr);
        self.set_nz(self.acc);
    }

    fn ldx(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::Immediate => {
                self.idx = self.fetch();
                self.set_nz(self.idx);
                return;
            }
            Mode::Absolute(offset) => self.get_absolute(offset).0,
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            _ => unimplemented!(),
        };

        self.idx = self.read_bus(addr);
        self.set_nz(self.idx);
    }

    fn ldy(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::Immediate => {
                self.idy = self.fetch();
                self.set_nz(self.idy);
                return;
            }
            Mode::Absolute(offset) => self.get_absolute(offset).0,
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            _ => unimplemented!(),
        };

        self.idy = self.read_bus(addr);
        self.set_nz(self.idy);
    }

    fn lsr(&mut self, mode: Mode) {
        match mode {
            Mode::Accumulator => {
                let byte = self.acc;
                self.acc = self.shift_byte(byte);
                self.set_nz(self.acc);
            }
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                let byte = self.read_bus(addr);
                let byte = self.shift_byte(byte);
                self.write_bus(addr, byte);
                self.set_nz(byte);
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
                self.set_nz(byte);
            }
            _ => unimplemented!("invalid addressing mode for LSR"),
        }
    }

    fn adc(&mut self, mode: Mode) {
        let byte = match mode {
            Mode::Immediate => self.fetch(),
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                self.read_bus(addr)
            }
            Mode::Absolute(offset) => {
                let addr = self.get_absolute(offset).0;
                self.read_bus(addr)
            }
            Mode::IndexedIndirect => {
                let addr = self.get_indexed_indirect();
                self.read_bus(addr)
            }
            Mode::IndirectIndexed => {
                let addr = self.get_indirect_indexed().0;
                self.read_bus(addr)
            }
            _ => unimplemented!("invalid addressing mode for ADC"),
        };

        let (res, carry) = self.acc.overflowing_add(byte);
        let overflow = self.check_overflow(self.acc, byte, res);
        self.acc = res;
        self.status.set_carry(carry);
        self.status.set_overflow(overflow);
        self.set_nz(self.acc);
    }
}

#[cfg(test)]
mod tests;
