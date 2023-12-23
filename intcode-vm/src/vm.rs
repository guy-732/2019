use std::str::FromStr;

use num::{Integer, ToPrimitive};

use crate::{
    error::{self, VMError},
    memory::Memory,
};

#[derive(Debug, Clone)]
pub struct IntcodeVM<T>
where
    T: Integer + Clone + ToPrimitive,
{
    memory: Memory<T>,
    instruction_ptr: usize,
}

impl<T> IntcodeVM<T>
where
    T: Integer + Clone + ToPrimitive,
{
    /// Creates a new VM from the given [`memory`](Memory)
    ///
    /// # Example
    ///
    /// ```
    /// # use intcode_vm::IntcodeVM;
    /// let vm = IntcodeVM::new([1, 0, 0, 3, 99]);
    /// ```
    #[inline]
    pub fn new<I: Into<Memory<T>>>(memory: I) -> Self {
        Self {
            memory: memory.into(),
            instruction_ptr: 0,
        }
    }

    /// Executes the intcode program in the memory of the VM
    ///
    /// When a halt instruction is encountered, returns [`Ok(())`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use intcode_vm::IntcodeVM;
    /// let mut vm = IntcodeVM::new([1, 0, 0, 3, 99]);
    /// vm.run().unwrap();
    /// ```
    ///
    /// Will return a [VMError] if a problem occurred, such as an unrecognized op code
    /// ```
    /// # use intcode_vm::IntcodeVM;
    /// let mut vm = IntcodeVM::new([15]); // 15 is not a valid op code
    /// assert!(vm.run().is_err());
    /// ```
    #[inline]
    pub fn run(&mut self) -> error::Result<(), T> {
        loop {
            let instruction = instr::Instruction::from_current_instr_ptr(self)?;
            let instruction_width = instruction.instruction_width();
            match instruction {
                instr::Instruction::Add(arg1, arg2, dest) => {
                    let arg1_val = self.memory.get(
                        arg1.to_usize()
                            .ok_or_else(|| VMError::CannotCastToUsize(arg1.clone()))?,
                    );
                    let arg2_val = self.memory.get(
                        arg2.to_usize()
                            .ok_or_else(|| VMError::CannotCastToUsize(arg2.clone()))?,
                    );
                    let destination_addr = dest
                        .to_usize()
                        .ok_or_else(|| VMError::CannotCastToUsize(dest.clone()))?;

                    let result = arg1_val.clone() + arg2_val.clone();
                    self.memory.set(destination_addr, result);
                }
                instr::Instruction::Mul(arg1, arg2, dest) => {
                    let arg1_val = self.memory.get(
                        arg1.to_usize()
                            .ok_or_else(|| VMError::CannotCastToUsize(arg1.clone()))?,
                    );
                    let arg2_val = self.memory.get(
                        arg2.to_usize()
                            .ok_or_else(|| VMError::CannotCastToUsize(arg2.clone()))?,
                    );
                    let destination_addr = dest
                        .to_usize()
                        .ok_or_else(|| VMError::CannotCastToUsize(dest.clone()))?;

                    let result = arg1_val.clone() * arg2_val.clone();
                    self.memory.set(destination_addr, result);
                }
                instr::Instruction::Halt => break,
            }

            self.increment_instr_ptr_by(instruction_width);
        }

        Ok(())
    }

    /// Returns the internal [Memory] of the VM
    ///
    /// # Example
    ///
    /// ```
    /// # use intcode_vm::IntcodeVM;
    /// let vm = IntcodeVM::from([1, 0, 0, 3, 99]);
    /// let memory = vm.into_memory();
    ///
    /// assert!(memory.memory_starts_with([1, 0, 0, 3, 99].iter()));
    /// ```
    #[inline]
    pub fn into_memory(self) -> Memory<T> {
        self.memory
    }

    #[inline]
    fn increment_instr_ptr_by(&mut self, incr: usize) {
        self.instruction_ptr += incr;
    }

    #[inline]
    fn get_at_instr_ptr(&self, offset: usize) -> &T {
        self.memory.get(self.instruction_ptr + offset)
    }

    #[inline]
    fn get_3_after_intr_ptr(&self) -> (&T, &T, &T) {
        (
            self.get_at_instr_ptr(1),
            self.get_at_instr_ptr(2),
            self.get_at_instr_ptr(3),
        )
    }
}

impl<T, I> From<I> for IntcodeVM<T>
where
    T: Integer + Clone + ToPrimitive,
    I: Into<Memory<T>>,
{
    #[inline]
    fn from(memory: I) -> Self {
        Self::new(memory)
    }
}

impl<T> FromStr for IntcodeVM<T>
where
    T: Integer + Clone + ToPrimitive + FromStr,
{
    type Err = <Memory<T> as FromStr>::Err;

    /// Parses the string into a instance of [Memory] (as per [`Memory::from_str()`](Memory::from_str))
    /// then build a VM from it
    ///
    /// # Example
    ///
    /// ```
    /// # use intcode_vm::IntcodeVM;
    /// let vm: IntcodeVM<i32> = "1,0,0,3,99".parse().unwrap();
    ///
    /// assert!(vm.into_memory().memory_starts_with([1, 0, 0, 3, 99].iter()));
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Memory<T>>().map(Self::new)
    }
}

mod instr {
    use num::{Integer, ToPrimitive};

    use crate::{
        error::{self, VMError},
        IntcodeVM,
    };

    #[derive(Debug, Clone)]
    pub(super) enum Instruction<'t, T> {
        Add(&'t T, &'t T, &'t T),
        Mul(&'t T, &'t T, &'t T),
        Halt,
    }

    impl<'t, T> Instruction<'t, T>
    where
        T: Integer + Clone + ToPrimitive,
    {
        pub(super) fn from_current_instr_ptr(vm: &'t IntcodeVM<T>) -> error::Result<Self, T> {
            let instr = vm.get_at_instr_ptr(0);
            match instr
                .to_u16()
                .ok_or_else(|| VMError::CannotCastToU16(instr.clone()))?
            {
                1 => Self::create_add(vm),
                2 => Self::create_mul(vm),
                99 => Ok(Self::Halt),
                other => Err(VMError::UnknownInstruction(other)),
            }
        }

        pub(super) const fn instruction_width(&self) -> usize {
            match self {
                Self::Add(_, _, _) => 4,
                Self::Mul(_, _, _) => 4,
                Self::Halt => 1,
            }
        }

        fn create_add(vm: &'t IntcodeVM<T>) -> error::Result<Self, T> {
            let (arg1, arg2, dest) = vm.get_3_after_intr_ptr();
            Ok(Self::Add(arg1, arg2, dest))
        }

        fn create_mul(vm: &'t IntcodeVM<T>) -> error::Result<Self, T> {
            let (arg1, arg2, dest) = vm.get_3_after_intr_ptr();
            Ok(Self::Mul(arg1, arg2, dest))
        }
    }
}
