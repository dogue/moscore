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

    pub fn dump_status_byte(&self) -> u8 {
        self.status.as_byte()
    }

    pub fn dump_status_flags(&self) -> Flags {
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

    fn bytes_from_addr(&self, addr: u16) -> (u8, u8) {
        let low = addr as u8;
        let high = (addr >> 8) as u8;
        return (low, high);
    }

    fn page_crossed(&self, byte: u8, index: u8) -> bool {
        (byte as u16) + (index as u16) > 255
    }

    fn shift_byte_right(&mut self, byte: u8) -> u8 {
        let shifted = byte >> 1;
        self.status.set_carry((byte & 0x1) != 0);
        self.clock_bus();
        shifted
    }

    fn shift_byte_left(&mut self, byte: u8) -> u8 {
        let shifted = byte << 1;
        self.status.set_carry((byte & 0x80) != 0);
        self.clock_bus();
        shifted
    }

    fn push_stack(&mut self, byte: u8) {
        self.sp = self.sp.wrapping_sub(1);
        let addr = self.addr_from_bytes(self.sp, 0x01);
        self.write_bus(addr, byte);
    }

    fn pull_stack(&mut self) -> u8 {
        let addr = self.addr_from_bytes(self.sp, 0x01);
        self.sp = self.sp.wrapping_add(1);
        // TODO(dogue): why are we clocking the bus here, but not in push_stack?
        self.clock_bus();
        self.read_bus(addr)
    }

    fn decode(&mut self, byte: u8) {
        match byte {
            0x01 => self.ora(Mode::IndexedIndirect),
            0x05 => self.ora(Mode::ZeroPage(Offset::None)),
            0x06 => self.asl(Mode::ZeroPage(Offset::None)),
            0x08 => self.php(),
            0x09 => self.ora(Mode::Immediate),
            0x0A => self.asl(Mode::Accumulator),
            0x0D => self.ora(Mode::Absolute(Offset::None)),
            0x0E => self.asl(Mode::Absolute(Offset::None)),
            0x11 => self.ora(Mode::IndirectIndexed),
            0x15 => self.ora(Mode::ZeroPage(Offset::X)),
            0x16 => self.asl(Mode::ZeroPage(Offset::X)),
            0x18 => self.clc(),
            0x19 => self.ora(Mode::Absolute(Offset::Y)),
            0x1D => self.ora(Mode::Absolute(Offset::X)),
            0x1E => self.asl(Mode::Absolute(Offset::X)),
            0x20 => self.jsr(),
            0x21 => self.and(Mode::IndexedIndirect),
            0x24 => self.bit(Mode::ZeroPage(Offset::None)),
            0x25 => self.and(Mode::ZeroPage(Offset::None)),
            0x26 => self.rol(Mode::ZeroPage(Offset::None)),
            0x28 => self.plp(),
            0x29 => self.and(Mode::Immediate),
            0x2A => self.rol(Mode::Accumulator),
            0x2C => self.bit(Mode::Absolute(Offset::None)),
            0x2D => self.and(Mode::Absolute(Offset::None)),
            0x2E => self.rol(Mode::Absolute(Offset::None)),
            0x31 => self.and(Mode::IndirectIndexed),
            0x35 => self.and(Mode::ZeroPage(Offset::X)),
            0x36 => self.rol(Mode::ZeroPage(Offset::X)),
            0x38 => self.sec(),
            0x39 => self.and(Mode::Absolute(Offset::Y)),
            0x3D => self.and(Mode::Absolute(Offset::X)),
            0x3E => self.rol(Mode::Absolute(Offset::X)),
            0x41 => self.eor(Mode::IndexedIndirect),
            0x45 => self.eor(Mode::ZeroPage(Offset::None)),
            0x46 => self.lsr(Mode::ZeroPage(Offset::None)),
            0x48 => self.pha(),
            0x49 => self.eor(Mode::Immediate),
            0x4A => self.lsr(Mode::Accumulator),
            0x4C => self.jmp(Mode::Absolute(Offset::None)),
            0x4D => self.eor(Mode::Absolute(Offset::None)),
            0x4E => self.lsr(Mode::Absolute(Offset::None)),
            0x51 => self.eor(Mode::IndirectIndexed),
            0x55 => self.eor(Mode::ZeroPage(Offset::X)),
            0x56 => self.lsr(Mode::ZeroPage(Offset::X)),
            0x58 => self.cli(),
            0x59 => self.eor(Mode::Absolute(Offset::Y)),
            0x5D => self.eor(Mode::Absolute(Offset::X)),
            0x5E => self.lsr(Mode::Absolute(Offset::X)),
            0x60 => self.rts(),
            0x61 => self.adc(Mode::IndexedIndirect),
            0x65 => self.adc(Mode::ZeroPage(Offset::None)),
            0x66 => self.ror(Mode::ZeroPage(Offset::None)),
            0x68 => self.pla(),
            0x69 => self.adc(Mode::Immediate),
            0x6A => self.ror(Mode::Accumulator),
            0x6C => self.jmp(Mode::Indirect),
            0x6D => self.adc(Mode::Absolute(Offset::None)),
            0x6E => self.ror(Mode::Absolute(Offset::None)),
            0x71 => self.adc(Mode::IndirectIndexed),
            0x75 => self.adc(Mode::ZeroPage(Offset::X)),
            0x76 => self.ror(Mode::ZeroPage(Offset::X)),
            0x78 => self.sei(),
            0x79 => self.adc(Mode::Absolute(Offset::Y)),
            0x7D => self.adc(Mode::Absolute(Offset::X)),
            0x7E => self.ror(Mode::Absolute(Offset::X)),
            0x81 => self.sta(Mode::IndexedIndirect),
            0x84 => self.sty(Mode::ZeroPage(Offset::None)),
            0x85 => self.sta(Mode::ZeroPage(Offset::None)),
            0x86 => self.stx(Mode::ZeroPage(Offset::None)),
            0x88 => self.dey(),
            0x8A => self.txa(),
            0x8C => self.sty(Mode::Absolute(Offset::None)),
            0x8D => self.sta(Mode::Absolute(Offset::None)),
            0x8E => self.stx(Mode::Absolute(Offset::None)),
            0x91 => self.sta(Mode::IndirectIndexed),
            0x94 => self.sty(Mode::ZeroPage(Offset::X)),
            0x95 => self.sta(Mode::ZeroPage(Offset::X)),
            0x96 => self.stx(Mode::ZeroPage(Offset::Y)),
            0x98 => self.tya(),
            0x99 => self.sta(Mode::Absolute(Offset::Y)),
            0x9A => self.txs(),
            0x9D => self.sta(Mode::Absolute(Offset::X)),
            0xA0 => self.ldy(Mode::Immediate),
            0xA1 => self.lda(Mode::IndexedIndirect),
            0xA2 => self.ldx(Mode::Immediate),
            0xA4 => self.ldy(Mode::ZeroPage(Offset::None)),
            0xA5 => self.lda(Mode::ZeroPage(Offset::None)),
            0xA6 => self.ldx(Mode::ZeroPage(Offset::None)),
            0xA8 => self.tay(),
            0xA9 => self.lda(Mode::Immediate),
            0xAA => self.tax(),
            0xAC => self.ldy(Mode::Absolute(Offset::None)),
            0xAD => self.lda(Mode::Absolute(Offset::None)),
            0xAE => self.ldx(Mode::Absolute(Offset::None)),
            0xB1 => self.lda(Mode::IndirectIndexed),
            0xB4 => self.ldy(Mode::ZeroPage(Offset::X)),
            0xB5 => self.lda(Mode::ZeroPage(Offset::X)),
            0xB6 => self.ldx(Mode::ZeroPage(Offset::Y)),
            0xB8 => self.clv(),
            0xB9 => self.lda(Mode::Absolute(Offset::Y)),
            0xBA => self.tsx(),
            0xBC => self.ldy(Mode::Absolute(Offset::X)),
            0xBD => self.lda(Mode::Absolute(Offset::X)),
            0xBE => self.ldx(Mode::Absolute(Offset::Y)),
            0xC0 => self.cpy(Mode::Immediate),
            0xC1 => self.cmp(Mode::IndexedIndirect),
            0xC4 => self.cpy(Mode::ZeroPage(Offset::None)),
            0xC5 => self.cmp(Mode::ZeroPage(Offset::None)),
            0xC6 => self.dec(Mode::ZeroPage(Offset::None)),
            0xC8 => self.iny(),
            0xC9 => self.cmp(Mode::Immediate),
            0xCA => self.dex(),
            0xCC => self.cpy(Mode::Absolute(Offset::None)),
            0xCD => self.cmp(Mode::Absolute(Offset::None)),
            0xCE => self.dec(Mode::Absolute(Offset::None)),
            0xD1 => self.cmp(Mode::IndirectIndexed),
            0xD5 => self.cmp(Mode::ZeroPage(Offset::X)),
            0xD6 => self.dec(Mode::ZeroPage(Offset::X)),
            0xD8 => self.cld(),
            0xD9 => self.cmp(Mode::Absolute(Offset::Y)),
            0xDD => self.cmp(Mode::Absolute(Offset::X)),
            0xDE => self.dec(Mode::Absolute(Offset::X)),
            0xE0 => self.cpx(Mode::Immediate),
            0xE4 => self.cpx(Mode::ZeroPage(Offset::None)),
            0xE6 => self.inc(Mode::ZeroPage(Offset::None)),
            0xE8 => self.inx(),
            0xEA => self.clock_bus(), // NOP
            0xEC => self.cpx(Mode::Absolute(Offset::None)),
            0xEE => self.inc(Mode::Absolute(Offset::None)),
            0xF6 => self.inc(Mode::ZeroPage(Offset::X)),
            0xF8 => self.sed(),
            0xFE => self.inc(Mode::Absolute(Offset::X)),
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

        let carry_bit = self.status.carry() as u8;

        let (partial, carry1) = byte.overflowing_add(carry_bit);
        let (res, carry2) = self.acc.overflowing_add(partial);
        let overflow = self.check_overflow(self.acc, byte, res);
        self.acc = res;
        self.status.set_carry(carry1 || carry2);
        self.status.set_overflow(overflow);
        self.set_nz(self.acc);
    }

    fn and(&mut self, mode: Mode) {
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
            _ => unimplemented!("invalid addressing mode for AND"),
        };

        self.acc &= byte;
        self.set_nz(self.acc);
    }

    fn asl(&mut self, mode: Mode) {
        match mode {
            Mode::Accumulator => {
                let byte = self.acc;
                self.acc = self.shift_byte_left(byte);
                self.set_nz(self.acc);
            }
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                let byte = self.read_bus(addr);
                let byte = self.shift_byte_left(byte);
                self.write_bus(addr, byte);
                self.set_nz(byte);
            }
            Mode::Absolute(offset) => {
                let (addr, crossed) = self.get_absolute(offset);
                if offset == Offset::X && !crossed {
                    self.clock_bus();
                }
                let byte = self.read_bus(addr);
                let byte = self.shift_byte_left(byte);
                self.write_bus(addr, byte);
                self.set_nz(byte);
            }
            _ => unimplemented!("invalid addressing mode for ASL"),
        }
    }

    fn _bcc() {}

    fn _bcs() {}

    fn _beq() {}

    fn bit(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::ZeroPage(_) => self.get_zeropage(Offset::None),
            Mode::Absolute(_) => self.get_absolute(Offset::None).0,
            _ => unimplemented!("invalid addressing mode for BIT"),
        };

        let byte = self.read_bus(addr);
        let res = byte & self.acc;
        self.status.set_negative(((byte) & (1 << 7)) != 0);
        self.status.set_overflow((byte & 1 << 6) != 0);
        self.status.set_zero(res == 0);
    }

    fn _bmi() {}

    fn _bne() {}

    fn _bpl() {}

    fn _brk() {}

    fn _bvc() {}

    fn _bvs() {}

    fn clc(&mut self) {
        self.status.set_carry(false);
        self.clock_bus();
    }

    fn cld(&mut self) {
        self.status.set_decimal(false);
        self.clock_bus();
    }

    fn cli(&mut self) {
        self.status.set_interrupt(false);
        self.clock_bus();
    }

    fn clv(&mut self) {
        self.status.set_overflow(false);
        self.clock_bus();
    }

    fn cmp(&mut self, mode: Mode) {
        let byte = match mode {
            Mode::Immediate => self.fetch(),
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                self.read_bus(addr)
            }
            Mode::Absolute(offset) => {
                let (addr, _) = self.get_absolute(offset);
                self.read_bus(addr)
            }
            Mode::IndexedIndirect => {
                let addr = self.get_indexed_indirect();
                self.read_bus(addr)
            }
            Mode::IndirectIndexed => {
                let (addr, _) = self.get_indirect_indexed();
                self.read_bus(addr)
            }
            _ => unimplemented!("invalid addressing mode for CMP"),
        };

        let res = self.acc.wrapping_sub(byte);
        self.set_nz(res);
        self.status.set_carry(self.acc >= byte);
    }

    fn cpx(&mut self, mode: Mode) {
        let byte = match mode {
            Mode::Immediate => self.fetch(),
            Mode::ZeroPage(_) => {
                let addr = self.get_zeropage(Offset::None);
                self.read_bus(addr)
            }
            Mode::Absolute(_) => {
                let (addr, _) = self.get_absolute(Offset::None);
                self.read_bus(addr)
            }
            _ => unimplemented!("invalid addressing mode for CPX"),
        };

        let res = self.idx.wrapping_sub(byte);
        self.set_nz(res);
        self.status.set_carry(self.idx >= byte);
    }

    fn cpy(&mut self, mode: Mode) {
        let byte = match mode {
            Mode::Immediate => self.fetch(),
            Mode::ZeroPage(_) => {
                let addr = self.get_zeropage(Offset::None);
                self.read_bus(addr)
            }
            Mode::Absolute(_) => {
                let (addr, _) = self.get_absolute(Offset::None);
                self.read_bus(addr)
            }
            _ => unimplemented!("invalid addressing mode for CPY"),
        };

        let res = self.idy.wrapping_sub(byte);
        self.set_nz(res);
        self.status.set_carry(self.idy >= byte);
    }

    fn dec(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            Mode::Absolute(offset) => {
                let (addr, crossed) = self.get_absolute(offset);
                if offset == Offset::X && !crossed {
                    self.clock_bus();
                }
                addr
            }
            _ => unimplemented!("invalid addressing mode for DEC"),
        };

        let byte = self.read_bus(addr);
        let byte = byte.wrapping_sub(1);
        self.clock_bus();
        self.write_bus(addr, byte);
        self.set_nz(byte);
    }

    fn dex(&mut self) {
        self.idx = self.idx.wrapping_sub(1);
        self.clock_bus();
        self.set_nz(self.idx);
    }

    fn dey(&mut self) {
        self.idy = self.idy.wrapping_sub(1);
        self.clock_bus();
        self.set_nz(self.idy);
    }

    fn eor(&mut self, mode: Mode) {
        let byte = match mode {
            Mode::Immediate => self.fetch(),
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                self.read_bus(addr)
            }
            Mode::Absolute(offset) => {
                let (addr, _) = self.get_absolute(offset);
                self.read_bus(addr)
            }
            Mode::IndexedIndirect => {
                let addr = self.get_indexed_indirect();
                self.read_bus(addr)
            }
            Mode::IndirectIndexed => {
                let (addr, _) = self.get_indirect_indexed();
                self.read_bus(addr)
            }
            _ => unimplemented!("invalid addressing mode for EOR"),
        };

        self.acc = self.acc ^ byte;
        self.set_nz(self.acc);
    }

    fn inc(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            Mode::Absolute(offset) => {
                let (addr, crossed) = self.get_absolute(offset);
                if offset == Offset::X && !crossed {
                    self.clock_bus();
                }
                addr
            }
            _ => unimplemented!("invalid addressing mode for INC"),
        };

        let byte = self.read_bus(addr);
        let byte = byte.wrapping_add(1);
        self.clock_bus();
        self.write_bus(addr, byte);
        self.set_nz(byte);
    }

    fn inx(&mut self) {
        self.idx = self.idx.wrapping_add(1);
        self.clock_bus();
        self.set_nz(self.idx);
    }

    fn iny(&mut self) {
        self.idy = self.idy.wrapping_add(1);
        self.clock_bus();
        self.set_nz(self.idy);
    }

    fn jmp(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::Absolute(_) => self.get_absolute(Offset::None).0,
            Mode::Indirect => {
                let i_low = self.fetch();
                let i_high = self.fetch();
                let indirect = self.addr_from_bytes(i_low, i_high);

                let t_low = self.read_bus(indirect);
                let t_high = self.read_bus(indirect.wrapping_add(1));
                let addr = self.addr_from_bytes(t_low, t_high);
                addr
            }
            _ => unimplemented!("invalid addressing mode for JMP"),
        };

        self.pc = addr;
    }

    fn jsr(&mut self) {
        let adl = self.fetch();
        let adh = self.fetch();
        let (pcl, pch) = self.bytes_from_addr(self.pc);
        self.push_stack(pch);
        self.push_stack(pcl);
        self.clock_bus();
        self.pc = self.addr_from_bytes(adl, adh);
    }

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
                self.acc = self.shift_byte_right(byte);
                self.set_nz(self.acc);
            }
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                let byte = self.read_bus(addr);
                let byte = self.shift_byte_right(byte);
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
                let byte = self.shift_byte_right(byte);
                self.write_bus(addr, byte);
                self.set_nz(byte);
            }
            _ => unimplemented!("invalid addressing mode for LSR"),
        }
    }

    fn ora(&mut self, mode: Mode) {
        let byte = match mode {
            Mode::Immediate => self.fetch(),
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                self.read_bus(addr)
            }
            Mode::Absolute(offset) => {
                let (addr, _) = self.get_absolute(offset);
                self.read_bus(addr)
            }
            Mode::IndexedIndirect => {
                let addr = self.get_indexed_indirect();
                self.read_bus(addr)
            }
            Mode::IndirectIndexed => {
                let (addr, _) = self.get_indirect_indexed();
                self.read_bus(addr)
            }
            _ => unimplemented!("invalid addressing mode for EOR"),
        };

        self.acc = self.acc | byte;
        self.set_nz(self.acc);
    }

    fn pha(&mut self) {
        self.clock_bus();
        self.push_stack(self.acc);
    }

    fn php(&mut self) {
        self.clock_bus();
        self.push_stack(self.status.as_byte());
    }

    fn pla(&mut self) {
        self.clock_bus();
        self.acc = self.pull_stack();
        self.set_nz(self.acc);
    }

    fn plp(&mut self) {
        self.clock_bus();
        let byte = self.pull_stack();
        self.status.from_byte(byte);
    }

    fn rol(&mut self, mode: Mode) {
        match mode {
            Mode::Accumulator => {
                // shift_byte_left() clobbers the carry flag
                let carry = self.status.carry() as u8;
                self.acc = self.shift_byte_left(self.acc) | carry;
                self.set_nz(self.acc);
            }
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                let carry = self.status.carry() as u8;
                let byte = self.read_bus(addr);
                let shifted = self.shift_byte_left(byte) | carry;
                self.write_bus(addr, shifted);
                self.set_negative(shifted);
            }
            Mode::Absolute(offset) => {
                let (addr, crossed) = self.get_absolute(offset);
                if offset == Offset::X && !crossed {
                    self.clock_bus();
                }
                let carry = self.status.carry() as u8;
                let byte = self.read_bus(addr);
                let shifted = self.shift_byte_left(byte) | carry;
                self.write_bus(addr, shifted);
                self.set_negative(shifted);
            }
            _ => unimplemented!("invalid addressng mode for ROR"),
        }
    }

    fn ror(&mut self, mode: Mode) {
        match mode {
            Mode::Accumulator => {
                // shift_byte_right() clobbers the carry flag
                let carry = self.status.carry() as u8;
                self.acc = self.shift_byte_right(self.acc) | carry << 7;
                self.set_nz(self.acc);
            }
            Mode::ZeroPage(offset) => {
                let addr = self.get_zeropage(offset);
                let carry = self.status.carry() as u8;
                let byte = self.read_bus(addr);
                let shifted = self.shift_byte_right(byte) | carry << 7;
                self.write_bus(addr, shifted);
                self.set_negative(shifted);
            }
            Mode::Absolute(offset) => {
                let (addr, crossed) = self.get_absolute(offset);
                if offset == Offset::X && !crossed {
                    self.clock_bus();
                }
                let carry = self.status.carry() as u8;
                let byte = self.read_bus(addr);
                let shifted = self.shift_byte_right(byte) | carry << 7;
                self.write_bus(addr, shifted);
                self.set_negative(shifted);
            }
            _ => unimplemented!("invalid addressng mode for ROR"),
        }
    }

    fn _rti() {}

    fn rts(&mut self) {
        let adl = self.pull_stack();
        let adh = self.pull_stack();
        self.clock_bus();
        self.pc = self.addr_from_bytes(adl, adh);
    }

    fn _sbc() {}

    fn sec(&mut self) {
        self.status.set_carry(true);
        self.clock_bus();
    }

    fn sed(&mut self) {
        self.status.set_decimal(true);
        self.clock_bus();
    }

    fn sei(&mut self) {
        self.status.set_interrupt(true);
        self.clock_bus();
    }

    fn sta(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            Mode::Absolute(offset) => {
                let (addr, crossed) = self.get_absolute(offset);
                if offset != Offset::None && !crossed {
                    self.clock_bus();
                }
                addr
            }
            Mode::IndexedIndirect => self.get_indexed_indirect(),
            Mode::IndirectIndexed => {
                let (addr, _) = self.get_indirect_indexed();
                self.clock_bus();
                addr
            }
            _ => unimplemented!("invalid addressing mode for STA"),
        };

        self.clock_bus();
        self.get_bus().write(addr, self.acc);
    }

    fn stx(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            Mode::Absolute(offset) => {
                let (addr, crossed) = self.get_absolute(offset);
                if offset != Offset::None && !crossed {
                    self.clock_bus();
                }
                addr
            }
            _ => unimplemented!("invalid addressing mode for STA"),
        };

        self.clock_bus();
        self.get_bus().write(addr, self.idx);
    }

    fn sty(&mut self, mode: Mode) {
        let addr = match mode {
            Mode::ZeroPage(offset) => self.get_zeropage(offset),
            Mode::Absolute(offset) => {
                let (addr, crossed) = self.get_absolute(offset);
                if offset != Offset::None && !crossed {
                    self.clock_bus();
                }
                addr
            }
            _ => unimplemented!("invalid addressing mode for STA"),
        };

        self.clock_bus();
        self.get_bus().write(addr, self.idy);
    }

    // I hate this abbreviation with my entire soul,
    // but consistency or something...
    fn tax(&mut self) {
        self.idx = self.acc;
        self.clock_bus();
        self.set_nz(self.idx);
    }

    fn tay(&mut self) {
        self.idy = self.acc;
        self.clock_bus();
        self.set_nz(self.idy);
    }

    fn tsx(&mut self) {
        self.idx = self.sp;
        self.clock_bus();
        self.set_nz(self.idy);
    }

    fn txa(&mut self) {
        self.acc = self.idx;
        self.clock_bus();
        self.set_nz(self.acc);
    }

    fn txs(&mut self) {
        self.sp = self.idx;
        self.clock_bus();
    }

    fn tya(&mut self) {
        self.acc = self.idy;
        self.clock_bus();
        self.set_nz(self.acc);
    }
}

#[cfg(test)]
mod tests;
