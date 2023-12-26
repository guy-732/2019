use num::{Integer, ToPrimitive};
use thiserror::Error;

/// [Error](std::error::Error) type returned by the [VM](crate::vm::IntcodeVM)
#[derive(Error, Debug)]
pub enum VMError<T>
where
    T: Integer + Clone + ToPrimitive,
{
    #[error("The instruction {0} was not any of [01, 02, 03, 04, 99]")]
    UnknownInstruction(u16),

    #[error("Could not cast {0} to u16 (opcode is cast to u16 before being parsed)")]
    CannotCastToU16(T),

    #[error("Could not cast {0} to usize (address is cast to usize before being used)")]
    CannotCastToUsize(T),

    #[error("The argument mode in opcode {opcode} for argument n°{arg_num} is not recognized (was {arg_mode}, should be 0 or 1)")]
    InvalidArgMode {
        opcode: u16,
        arg_num: u8,
        arg_mode: u8,
    },

    #[error(
        "The argument mode in opcode {opcode} for argument n°{arg_num} cannot be immediate (1)"
    )]
    ArgModeCannotBeImmediate { opcode: u16, arg_num: u8 },
}

pub type Result<T, I> = std::result::Result<T, VMError<I>>;
