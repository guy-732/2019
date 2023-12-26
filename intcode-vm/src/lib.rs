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

    #[test]
    fn test_io() {
        let mut vm = IntcodeVM::from([3, 0, 4, 0, 99]);
        assert_eq!(vm.run().unwrap(), VMResult::WaitingForInput);
        assert_eq!(vm.set_next_input(12345), None);
        assert_eq!(vm.run().unwrap(), VMResult::Output(12345));
        assert_eq!(vm.run().unwrap(), VMResult::Halted);
        assert!(vm.into_memory().memory_starts_with(&[12345, 0, 4, 0, 99]));
    }

    #[test]
    fn test_io_immediate() {
        let mut vm = IntcodeVM::from([3, 3, 104, 0, 99]);
        assert_eq!(vm.run().unwrap(), VMResult::WaitingForInput);
        assert_eq!(vm.set_next_input(12345), None);
        assert_eq!(vm.run().unwrap(), VMResult::Output(12345));
        assert_eq!(vm.get_next_input(), &None);
        assert_eq!(vm.run().unwrap(), VMResult::Halted);
        assert!(vm.into_memory().memory_starts_with(&[3, 3, 104, 12345, 99]));
    }

    #[test]
    fn test_immediate_mul_into_halt() {
        let mut vm = IntcodeVM::from([1002, 4, 3, 4, 33]);
        assert_eq!(vm.run().unwrap(), VMResult::Halted);
        assert!(vm.into_memory().memory_starts_with(&[1002, 4, 3, 4, 99]));
    }

    #[test]
    fn test_negative_add() {
        let mut vm = IntcodeVM::from([1101, 100, -1, 4, 0]);
        assert_eq!(vm.run().unwrap(), VMResult::Halted);
        assert!(vm.into_memory().memory_starts_with(&[1101, 100, -1, 4, 99]));
    }
}
