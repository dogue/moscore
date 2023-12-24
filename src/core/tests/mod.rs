use crate::traits::Bus;

mod adc;
mod lda;
mod ldx;
mod ldy;
mod lsr;

#[derive(Debug, Clone)]
struct MockBus {
    mem: [u8; 0xffff],
}

impl MockBus {
    pub fn new() -> Self {
        Self { mem: [0; 0xffff] }
    }
}

impl Bus for MockBus {
    fn read(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.mem[addr as usize] = byte;
    }

    fn on_clock(&mut self) {
        self.mem[0xc10c] += 1;
    }

    fn load_rom(&mut self, prog: Vec<u8>) -> Result<(), crate::error::BusError> {
        prog.into_iter()
            .enumerate()
            .for_each(|(i, byte)| self.mem[i] = byte);

        Ok(())
    }

    fn dump_rom(&self) -> Vec<u8> {
        self.mem.to_vec()
    }
}
