use std::cell::{Ref, RefMut};
#[allow(arithmetic_overflow)]
use std::{cell::RefCell, rc::Rc};

use crate::{error::CoreError, traits::Bus};

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

    pub fn dump(&self) {
        println!("ACC: {:x}", self.acc);
        println!("IDX: {:x}", self.idx);
        println!("IDY: {:x}", self.idy);
        println!("SP: {:x}", self.sp);
        println!("PC: {:x}", self.pc);
        println!("FLAGS: {:0>8b}", self.status);
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
        let mut bus = self.bus.borrow_mut();
        let byte = bus.read(addr);
        bus.on_clock();
        byte
    }

    fn fetch(&mut self) -> u8 {
        let mut bus = self.bus.borrow_mut();
        let byte = bus.read(self.pc);
        bus.on_clock();
        self.pc += 1;
        byte
    }

    fn addr_from_bytes(&self, low: u8, high: u8) -> u16 {
        (u16::from(high) << 8) | u16::from(low)
    }

    fn decode(&mut self, byte: u8) {
        match byte {
            0xA9 => self.load_a_immediate(),
            0xAD => self.load_a_absolute(),
            _ => self.halted = true,
        }
    }

    fn load_a_immediate(&mut self) {
        self.acc = self.fetch();
    }

    fn load_a_zeropage(&mut self) {
        let low = self.fetch();
        let high = 0x00;
        let addr = self.addr_from_bytes(low, high);
        self.acc = self.read_bus(addr);
    }

    fn load_a_absolute(&mut self) {
        let low = self.fetch();
        let high = self.fetch();
        let addr = self.addr_from_bytes(low, high);

        self.acc = self.read_bus(addr);
    }
}
