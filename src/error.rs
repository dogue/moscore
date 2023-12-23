use thiserror::Error;

#[derive(Debug, Error)]
pub enum BusError {
    #[error("program too large: ROM size: {rom_size}, program size: {prog_size}")]
    ProgramTooLarge { rom_size: usize, prog_size: usize },
}

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("problem initializing the bus: {0}")]
    BusInitFailed(#[from] BusError),
}
