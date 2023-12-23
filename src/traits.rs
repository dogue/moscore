use crate::error::BusError;
use std::fmt::Debug;

/// The `Bus` trait defines the interface for the system bus.
///
/// It handles both the address and the data bus for communication
/// between the [`Core`][super::core::Core], memory, and I/O devices.
/// Implementors of this trait should provide, at minimum, a ROM space
/// to store program bytes and a RAM space for the stack and variables.
///
/// ROM is assumed to be mapped at the end of the address space, per the
/// reset sequence of the 6502.
pub trait Bus: 'static + Debug {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, byte: u8);
    fn on_clock(&mut self);
    fn load_rom(&mut self, prog: Vec<u8>) -> Result<(), BusError>;
    fn dump_rom(&self) -> Vec<u8>;
}
