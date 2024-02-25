use crate::traits::Bus;

use super::Core;

mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod brk;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod eor;
mod inc;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
mod lsr;
mod ora;
mod push_pull;
mod rol;
mod ror;
mod sbc;
mod sec;
mod sed;
mod sei;
mod sta;
mod stx;
mod sty;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

#[derive(Debug, Clone)]
struct MockBus {
    mem: [u8; 0x10000],
}

impl MockBus {
    pub fn new() -> Self {
        Self { mem: [0; 0x10000] }
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

pub fn verify_clocks(core: &Core, expected: i32) -> bool {
    let clocks = core.get_bus().read(0xc10c);
    dbg!(clocks);
    return (clocks as i32) == expected;
}
