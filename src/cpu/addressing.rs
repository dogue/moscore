#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Accumulator,
    Absolute(Offset),
    Immediate,
    Implied,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
    Relative,
    ZeroPage(Offset),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Offset {
    None,
    X,
    Y,
}
