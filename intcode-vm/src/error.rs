use num::{Integer, ToPrimitive};
use thiserror::Error;

/// [Error](std::error::Error) type returned by the [VM](crate::vm::IntcodeVM)
#[derive(Error, Debug)]
pub enum VMError<T>
where
    T: Integer + Clone + ToPrimitive,
{
    #[error("The instruction {0} was not any of [01, 02, 99]")]
    UnknownInstruction(u16),

    #[error("Could not cast {0} to u16")]
    CannotCastToU16(T),

    #[error("Could not cast {0} to usize")]
    CannotCastToUsize(T),
}

pub type Result<T, I> = std::result::Result<T, VMError<I>>;
