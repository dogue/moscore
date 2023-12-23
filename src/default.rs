use crate::{error::BusError, traits::Bus};

#[derive(Debug)]
pub struct DefaultBus {
    ram: [u8; 0x7fff],
    rom: [u8; 0x7fff],
}

impl Bus for DefaultBus {
    fn read(&mut self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            0x0000..=0x7fff => self.ram[addr],
            0x8000..=0xffff => self.rom[addr - 0x8000],
            _ => unreachable!(),
        }
    }

    fn write(&mut self, addr: u16, byte: u8) {
        let addr = addr as usize;
        match addr {
            0x0000..=0x7fff => self.ram[addr] = byte,
            0x8000..=0xffff => self.rom[addr - 0x8000] = byte,
            _ => unreachable!(),
        }
    }

    // no peripherals lmao
    fn on_clock(&mut self) {}

    fn load_rom(&mut self, prog: Vec<u8>) -> Result<(), crate::error::BusError> {
        if prog.len() > self.rom.len() {
            return Err(BusError::ProgramTooLarge {
                rom_size: self.rom.len(),
                prog_size: prog.len(),
            });
        }

        prog.into_iter()
            .enumerate()
            .for_each(|(i, byte)| self.rom[i] = byte);

        Ok(())
    }

    fn dump_rom(&self) -> Vec<u8> {
        self.rom.to_vec()
    }
}

impl Default for DefaultBus {
    fn default() -> Self {
        Self {
            ram: [0; 0x7fff],
            rom: [0; 0x7fff],
        }
    }
}
