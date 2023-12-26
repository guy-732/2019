//! Implementation of [Advent of Code's 2019](https://adventofcode.com/2019) Intcode VM
//!
//! # Examples
//!
//! ```
//! use intcode_vm::{IntcodeVM, VMResult};
//!
//! let mut vm = IntcodeVM::new([1, 0, 0, 3, 99]);
//! assert_eq!(vm.run().unwrap(), VMResult::Halted);
//! assert!(vm.into_memory().memory_starts_with(&[1, 0, 0, 2, 99]))
//! ```
//!
//! ```
//! # use intcode_vm::{IntcodeVM, VMResult};
//! let mut vm = IntcodeVM::new([
//!     1, 9, 10, 3, // add values at position 9 and 10 together and put that at position 3
//!     2, 3, 11, 0, // multiplies values at position 3 and 11 together and put that at position 0
//!     99,          // halts
//!     30, 40, 50   // data, not instructions
//! ]);
//!
//! assert_eq!(vm.run().unwrap(), VMResult::Halted);
//! assert!(vm.into_memory().memory_starts_with(&[
//!     3500, 9, 10, 70,
//!     2, 3, 11, 0,
//!     99,
//!     30, 40, 50
//! ]))
//! ```

pub mod error;
pub mod memory;
pub mod vm;

pub use vm::IntcodeVM;
pub use vm::VMResult;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut vm = IntcodeVM::new([1, 0, 0, 0, 99]);
        assert_eq!(vm.run().unwrap(), VMResult::Halted);
        assert!(vm.into_memory().memory_starts_with(&[2, 0, 0, 0, 99]));
    }

    #[test]
    fn test_mul1() {
        let mut vm = IntcodeVM::new([2, 3, 0, 3, 99]);
        assert_eq!(vm.run().unwrap(), VMResult::Halted);
        assert!(vm.into_memory().memory_starts_with(&[2, 3, 0, 6, 99]));
    }

    #[test]
    fn test_mul2() {
        let mut vm = IntcodeVM::new([2, 4, 4, 5, 99, 0]);
        assert_eq!(vm.run().unwrap(), VMResult::Halted);
        assert!(vm.into_memory().memory_starts_with(&[2, 4, 4, 5, 99, 9801]));
    }

    #[test]
    fn test_add_into_mul_dynamic() {
        let mut vm = IntcodeVM::new([1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(vm.run().unwrap(), VMResult::Halted);
        assert!(vm
            .into_memory()
            .memory_starts_with(&[30, 1, 1, 4, 2, 5, 6, 0, 99]));
    }
}
