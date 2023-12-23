use crate::{error::CoreError, traits::Bus};
#[allow(arithmetic_overflow)]
use std::{cell::RefCell, cell::RefMut, rc::Rc};

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
        let byte = self.bus.borrow_mut().read(addr);
        self.clock_bus();
        byte
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

    fn decode(&mut self, byte: u8) {
        match byte {
            0xA9 => self.lda_immediate(),
            0xA5 => self.lda_zeropage(),
            0xB5 => self.lda_zeropage_x(),
            0xAD => self.lda_absolute(),
            0xBD => self.lda_absolute_indexed(self.idx),
            0xB9 => self.lda_absolute_indexed(self.idy),
            0xA1 => self.lda_indirect_x(),
            0xB1 => self.lda_indirect_y(),
            _ => self.halted = true,
        }
    }
}

// LDA
impl Core {
    // 0xA9
    fn lda_immediate(&mut self) {
        self.acc = self.fetch();
    }

    // 0xA5
    fn lda_zeropage(&mut self) {
        let low = self.fetch();
        let high = 0x00;
        let addr = self.addr_from_bytes(low, high);
        self.acc = self.read_bus(addr);
    }

    // 0xB5
    fn lda_zeropage_x(&mut self) {
        let low = self.fetch() + self.idx;
        self.clock_bus();
        let addr = self.addr_from_bytes(low, 0x00);
        self.acc = self.read_bus(addr);
    }

    // 0xAD
    fn lda_absolute(&mut self) {
        let low = self.fetch();
        let high = self.fetch();
        let addr = self.addr_from_bytes(low, high);

        self.acc = self.read_bus(addr);
    }

    // 0xBD, 0xB9
    fn lda_absolute_indexed(&mut self, index: u8) {
        let low = self.fetch();
        let high = self.fetch();
        let mut addr = self.addr_from_bytes(low, high);
        addr += index as u16;
        if self.page_crossed(low, index) {
            self.clock_bus();
        }
        self.acc = self.read_bus(addr);
    }

    // A1
    fn lda_indirect_x(&mut self) {
        let byte = self.fetch() + self.idx;
        let addr = self.addr_from_bytes(byte, 0x00);
        let low = self.read_bus(addr);
        let high = self.read_bus(addr + 1);
        self.clock_bus();
        let indirect = self.addr_from_bytes(low, high);
        self.acc = self.read_bus(indirect);
    }

    // B1
    fn lda_indirect_y(&mut self) {
        let byte = self.fetch();
        let addr = self.addr_from_bytes(byte, 0x00);
        let low = self.read_bus(addr);
        let high = self.read_bus(addr + 1);
        if self.page_crossed(low, self.idy) {
            self.clock_bus();
        }
        let indirect = self.addr_from_bytes(low, high) + self.idy as u16;
        self.acc = self.read_bus(indirect);
    }
}
