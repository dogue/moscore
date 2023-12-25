#![allow(dead_code)]
#[derive(Debug)]
pub struct Flags {
    carry: bool,
    zero: bool,
    interrupt_disable: bool,
    decimal_mode: bool,
    break_command: bool,
    overflow: bool,
    negative: bool,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal_mode: false,
            break_command: false,
            overflow: false,
            negative: false,
        }
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }

    pub fn carry(&self) -> bool {
        self.carry
    }

    pub fn set_zero(&mut self, value: bool) {
        self.zero = value;
    }

    pub fn zero(&self) -> bool {
        self.zero
    }

    pub fn set_interrupt(&mut self, value: bool) {
        self.interrupt_disable = value;
    }

    pub fn interrupt(&self) -> bool {
        self.interrupt_disable
    }

    pub fn set_decimal(&mut self, value: bool) {
        self.decimal_mode = value;
    }

    pub fn decimal(&self) -> bool {
        self.decimal_mode
    }

    pub fn set_break(&mut self, value: bool) {
        self.break_command = value;
    }

    pub fn break_cmd(&self) -> bool {
        self.break_command
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.overflow = value;
    }

    pub fn overflow(&self) -> bool {
        self.overflow
    }

    pub fn set_negative(&mut self, value: bool) {
        self.negative = value;
    }

    pub fn negative(&self) -> bool {
        self.negative
    }

    pub fn as_byte(&self) -> u8 {
        (self.carry as u8)
            | ((self.zero as u8) << 1)
            | ((self.interrupt_disable as u8) << 2)
            | ((self.decimal_mode as u8) << 3)
            | ((self.break_command as u8) << 4)
            | ((self.overflow as u8) << 6)
            | ((self.negative as u8) << 7)
    }

    pub fn from_byte(&mut self, byte: u8) {
        self.carry = byte & 0x01 != 0;
        self.zero = byte & 0x02 != 0;
        self.interrupt_disable = byte & 0x04 != 0;
        self.decimal_mode = byte & 0x08 != 0;
        self.break_command = byte & 0x10 != 0;
        self.overflow = byte & 0x40 != 0;
        self.negative = byte & 0x80 != 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as_byte() {
        let mut f = Flags::new();
        f.set_carry(true);
        f.set_negative(true);
        let byte = f.as_byte();

        assert_eq!(byte, 0b1000_0001);
    }

    #[test]
    fn test_from_byte() {
        let byte = 0b1000_0011;
        let mut f = Flags::new();
        f.from_byte(byte);

        assert_eq!(f.carry, true);
        assert_eq!(f.negative, true);
        assert_eq!(f.zero, true);
    }
}
