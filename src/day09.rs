use std::error::Error;

use intcode_vm::{IntcodeVM, VMResult};

#[inline]
fn solve(input: &str, part: i64) -> Result<i64, Box<dyn Error>> {
    let mut vm: IntcodeVM<_> = input.parse()?;
    vm.set_next_input(part);

    let result = match vm.run()? {
        VMResult::Output(out) => out,
        other => Err(format!("Expected an output, got {:?}", other))?,
    };

    assert_eq!(vm.run()?, VMResult::Halted);
    Ok(result)
}

#[aoc(day09, part1)]
fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    solve(input, 1)
}

#[aoc(day09, part2)]
fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    solve(input, 2)
}
