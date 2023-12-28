use std::{error::Error, num::ParseIntError};

use intcode_vm::{IntcodeVM, VMResult};
use itertools::Itertools;

#[aoc_generator(day07)]
fn parse(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[aoc(day07, part1)]
fn part1(program: &[i64]) -> Result<i64, Box<dyn Error>> {
    (0..=4)
        .permutations(5)
        .map(|settings| {
            let mut current_trust = 0;
            for phase in settings {
                let mut vm = IntcodeVM::from(program.iter().copied());
                vm.set_next_input(phase);

                match vm.run()? {
                    VMResult::WaitingForInput => (),
                    other => Err(format!("Expected variant WaitingForInput, got {:?}", other))?,
                }

                vm.set_next_input(current_trust);
                match vm.run()? {
                    VMResult::Output(produced) => current_trust = produced,
                    other => Err(format!("Expected variant Output(value), got {:?}", other))?,
                }

                match vm.run()? {
                    VMResult::Halted => (),
                    other => Err(format!("Expected variant Halted, got {:?}", other))?,
                }
            }

            Ok::<_, Box<dyn Error>>(current_trust)
        })
        .fold_ok(i64::MIN, std::cmp::max)
}

#[aoc(day07, part2)]
fn part2(program: &[i64]) -> Result<i64, Box<dyn Error>> {
    (5..=9)
        .permutations(5)
        .map(|settings| -> Result<i64, Box<dyn Error>> {
            let mut vms = settings
                .into_iter()
                .map(|phase| {
                    let mut vm = IntcodeVM::from(program.iter().copied());
                    vm.set_next_input(phase);
                    vm
                })
                .collect_vec();

            let mut current_thrust = 0;
            Ok('outer: loop {
                for vm in vms.iter_mut() {
                    match vm.run()? {
                        VMResult::WaitingForInput => (),
                        VMResult::Halted => break 'outer current_thrust,
                        VMResult::Output(v) => Err(format!("VM outputed {v} unexpectedly"))?,
                    }

                    vm.set_next_input(current_thrust);
                    match vm.run()? {
                        VMResult::Output(out) => current_thrust = out,
                        other => Err(format!("Expected variant Output(value), got {:?}", other))?,
                    }
                }
            })
        })
        .fold_ok(i64::MIN, std::cmp::max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            part1(&parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").unwrap()).unwrap(),
            43210
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            part1(
                &parse("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
                    .unwrap()
            )
            .unwrap(),
            54321
        );
    }

    #[test]
    fn part1_example3() {
        assert_eq!(
            part1(
                &parse("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")
                .unwrap()
            )
            .unwrap(),
            65210
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            part2(
                &parse("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")
                .unwrap()
            )
            .unwrap(),
            139629729
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            part2(
                &parse("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10")
                .unwrap()
            )
            .unwrap(),
            18216
        );
    }
}
