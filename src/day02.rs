use std::error::Error;

use intcode_vm::{memory::Memory, IntcodeVM};

#[aoc(day02, part1)]
fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut memory = input.parse::<Memory<_>>()?;
    memory.set(1, 12);
    memory.set(2, 2);

    let mut vm = IntcodeVM::new(memory);
    vm.run()?;

    Ok(*vm.into_memory().get(0))
}

#[aoc(day02, part2)]
fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    const TARGET_RESULT: i64 = 19690720;
    let memory = input.parse::<Memory<_>>()?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = memory.clone();
            memory.set(1, noun);
            memory.set(2, verb);

            let mut vm = IntcodeVM::new(memory);
            vm.run()?;

            if *vm.into_memory().get(0) == TARGET_RESULT {
                return Ok(100 * noun + verb);
            }
        }
    }

    unreachable!("The loop should return")
}
