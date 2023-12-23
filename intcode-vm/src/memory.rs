use std::{ops::Index, str::FromStr};

use num::{Integer, ToPrimitive};

/// Represents a [VM's](crate::vm::IntcodeVM) memory
///
/// # Example
///
/// ```
/// # use intcode_vm::{memory::Memory, IntcodeVM};
/// // initializes memory with values
/// let memory = Memory::from([1, 0, 0, 3, 99]);
///
/// // do something with it before passing it to the VM
/// let vm = IntcodeVM::new(memory);
/// ```
#[derive(Debug, Clone)]
pub struct Memory<T>
where
    T: Integer + Clone + ToPrimitive,
{
    mem: Vec<T>,
}

impl<T> Memory<T>
where
    T: Integer + Clone + ToPrimitive,
{
    /// Returns a reference to the value at `address` in the memory
    ///
    /// # Note
    /// The returned value is the same as [`Memory::index()`](Memory::index)
    /// i.e.
    /// ```
    /// # use intcode_vm::memory::Memory;
    /// let memory = Memory::from([1, 0, 0, 3, 99]);
    ///
    /// assert_eq!(memory.get(4), &memory[4]);
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # use intcode_vm::memory::Memory;
    /// let memory = Memory::from([1, 0, 0, 3, 99]);
    ///
    /// assert_eq!(memory.get(0), &1);
    /// assert_eq!(memory.get(1), &0);
    /// assert_eq!(memory.get(2), &0);
    /// assert_eq!(memory.get(3), &3);
    /// assert_eq!(memory.get(4), &99);
    /// ```
    ///
    /// # Panics
    /// if the memory address does not exist
    /// ```rust,should_panic
    /// # use intcode_vm::memory::Memory;
    /// let memory = Memory::from([1, 0, 0, 3, 99]);
    ///
    /// memory.get(10); // panics
    /// ```
    #[inline]
    pub fn get(&self, address: usize) -> &T {
        self.mem.get(address).expect("Out of bound memory access")
    }

    /// Replaces the value at `address` with `value`
    ///
    /// # Example
    ///
    /// ```
    /// # use intcode_vm::memory::Memory;
    /// let mut memory = Memory::from([1, 0, 0, 3, 99]);
    ///
    /// memory.set(3, 2);
    ///
    /// assert_eq!(memory[0], 1);
    /// assert_eq!(memory[1], 0);
    /// assert_eq!(memory[2], 0);
    /// assert_eq!(memory[3], 2);
    /// assert_eq!(memory[4], 99);
    /// ```
    ///
    /// # Panics
    /// if the memory address does not exist
    /// ```rust,should_panic
    /// # use intcode_vm::memory::Memory;
    /// let mut memory = Memory::from([1, 0, 0, 3, 99]);
    ///
    /// memory.set(10, 2); // panics
    /// ```
    #[inline]
    pub fn set(&mut self, address: usize, value: T) {
        *self
            .mem
            .get_mut(address)
            .expect("Out of bound memory access") = value;
    }

    /// Creates an [iterator](Iterator) over the memory
    ///
    /// # Example
    ///
    /// ```
    /// # use intcode_vm::memory::Memory;
    /// let memory = Memory::from([1, 0, 0, 3, 99]);
    /// let mut iter = memory.iter();
    ///
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&99));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.mem.iter()
    }

    /// Checks if this memory's first `n` elements are the same as the `n` elements of `iter`
    /// (`n` being the number of elements in `iter`).
    ///
    /// # Example
    /// ```
    /// # use intcode_vm::memory::Memory;
    /// let memory = Memory::from([1, 0, 0, 3, 99]);
    ///
    /// assert!(memory.memory_starts_with([1, 0, 0, 3, 99].iter()));
    /// assert!(memory.memory_starts_with([1, 0, 0].iter())); // shorter than memory, but still valid
    /// assert!(!memory.memory_starts_with([1, 0, 0, 3, 99, 5].iter())); // longer than memory, not valid
    /// assert!(!memory.memory_starts_with([1, 0, 0, 0, 99].iter())); // element at index 3 is different
    /// ```
    #[inline]
    pub fn memory_starts_with<'t, I>(&self, iter: I) -> bool
    where
        T: 't,
        I: IntoIterator<Item = &'t T>,
    {
        let mut self_iter = self.iter();
        iter.into_iter().all(|iter_val| {
            self_iter
                .next()
                .map_or(false, |self_val| self_val.eq(iter_val))
        })
    }
}

impl<T> Index<usize> for Memory<T>
where
    T: Integer + Clone + ToPrimitive,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<T> FromIterator<T> for Memory<T>
where
    T: Integer + Clone + ToPrimitive,
{
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self {
        Self {
            mem: iter.into_iter().collect(),
        }
    }
}

impl<T, I> From<I> for Memory<T>
where
    T: Integer + Clone + ToPrimitive,
    I: IntoIterator<Item = T>,
{
    #[inline]
    fn from(value: I) -> Self {
        value.into_iter().collect()
    }
}

impl<T> FromStr for Memory<T>
where
    T: Integer + Clone + ToPrimitive + FromStr,
{
    type Err = <T as FromStr>::Err;

    /// Parses a comma separated list of values **WITHOUT SPACES**
    /// (unless [`T::from_str()`](FromStr) ignores them)
    ///
    /// # Example
    ///
    /// ```
    /// # use intcode_vm::memory::Memory;
    /// let memory: Memory<i32> = "1,0,0,3,99".parse().unwrap();
    ///
    /// assert!(memory.memory_starts_with([1, 0, 0, 3, 99].iter()));
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',').map(|part| part.parse::<T>()).collect()
    }
}
