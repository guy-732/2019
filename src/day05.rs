use std::error::Error;

use intcode_vm::{IntcodeVM, VMResult};

#[aoc(day05, part1)]
fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut last_diagnostic = 0;
    let mut vm: IntcodeVM<_> = input.parse()?;
    vm.set_next_input(1);

    loop {
        match vm.run()? {
            VMResult::WaitingForInput => {
                return Err("VM asked for input beyond the `1` already provided".into());
            }
            VMResult::Output(diagnostic) => {
                if last_diagnostic != 0 {
                    // panics instead of returning an error in case I need to debug (with a breakpoint on panic)
                    panic!("Last diagnostics wasn't 0 (was {})", last_diagnostic);
                }

                last_diagnostic = diagnostic;
            }
            VMResult::Halted => {
                return Ok(last_diagnostic);
            }
        }
    }
}

#[aoc(day05, part2)]
fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut vm: IntcodeVM<_> = input.parse()?;
    vm.set_next_input(5);

    let result = match vm.run()? {
        VMResult::Output(out) => out,
        VMResult::WaitingForInput => Err("VM asked for input beyond the `5` already provided")?,
        VMResult::Halted => Err("VM halted without producing any values")?,
    };

    assert_eq!(vm.run()?, VMResult::Halted);
    Ok(result)
}
