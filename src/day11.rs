use std::{error::Error, iter};

use fnv::FnvHashMap;
use intcode_vm::{IntcodeVM, VMResult};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum PanelColor {
    #[default]
    Black,
    White,
}

impl From<PanelColor> for char {
    #[inline]
    fn from(value: PanelColor) -> Self {
        match value {
            PanelColor::Black => ' ',
            PanelColor::White => '#',
        }
    }
}

impl PanelColor {
    #[inline]
    const fn as_i64(&self) -> i64 {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
}

type Position = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

impl Direction {
    #[inline]
    const fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    #[inline]
    const fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West,
            Self::East => Self::South,
        }
    }

    #[inline]
    const fn translate_pos(&self, pos: Position) -> Position {
        match self {
            Self::North => (pos.0 - 1, pos.1),
            Self::South => (pos.0 + 1, pos.1),
            Self::East => (pos.0, pos.1 + 1),
            Self::West => (pos.0, pos.1 - 1),
        }
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut vm: IntcodeVM<i64> = input.parse()?;
    let mut painted_map = FnvHashMap::default();
    let mut current_pos: Position = (0, 0);
    let mut current_direction = Direction::North;

    loop {
        match vm.run()? {
            VMResult::Halted => break,
            VMResult::WaitingForInput => {
                vm.set_next_input(painted_map.get(&current_pos).map_or(0, PanelColor::as_i64));
            }
            VMResult::Output(out) => panic!("Expected program to ask for input, outputed {}", out),
        }

        match vm.run()? {
            VMResult::Halted => break,
            VMResult::WaitingForInput => panic!("Program want another input... but we're waiting for the color to paint the current panel"),
            VMResult::Output(paint) => {
                painted_map.insert(
                    current_pos,
                    match paint {
                        0 => PanelColor::Black,
                        1 => PanelColor::White,
                        other => panic!("Color to paint was neither 0 nor 1 ({})", other),
                    },
                );
            }
        }

        match vm.run()? {
            VMResult::Halted => break,
            VMResult::WaitingForInput => panic!(
                "Program want another input... but we're waiting for which direction we turn to"
            ),
            VMResult::Output(turn) => match turn {
                0 => {
                    current_direction = current_direction.turn_left();
                }
                1 => {
                    current_direction = current_direction.turn_right();
                }
                other => panic!("Direction to turn to was neither 0 nor 1 ({})", other),
            },
        }

        current_pos = current_direction.translate_pos(current_pos);
    }

    Ok(painted_map.len())
}

#[aoc(day11, part2)]
fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut vm: IntcodeVM<i64> = input.parse()?;
    let mut current_pos: Position = (0, 0);
    let mut current_direction = Direction::North;
    let mut painted_map =
        iter::once((current_pos, PanelColor::White)).collect::<FnvHashMap<_, _>>();

    loop {
        match vm.run()? {
            VMResult::Halted => break,
            VMResult::WaitingForInput => {
                vm.set_next_input(painted_map.get(&current_pos).map_or(0, PanelColor::as_i64));
            }
            VMResult::Output(out) => panic!("Expected program to ask for input, outputed {}", out),
        }

        match vm.run()? {
            VMResult::Halted => break,
            VMResult::WaitingForInput => panic!("Program want another input... but we're waiting for the color to paint the current panel"),
            VMResult::Output(paint) => {
                painted_map.insert(
                    current_pos,
                    match paint {
                        0 => PanelColor::Black,
                        1 => PanelColor::White,
                        other => panic!("Color to paint was neither 0 nor 1 ({})", other),
                    },
                );
            }
        }

        match vm.run()? {
            VMResult::Halted => break,
            VMResult::WaitingForInput => panic!(
                "Program want another input... but we're waiting for which direction we turn to"
            ),
            VMResult::Output(turn) => match turn {
                0 => {
                    current_direction = current_direction.turn_left();
                }
                1 => {
                    current_direction = current_direction.turn_right();
                }
                other => panic!("Direction to turn to was neither 0 nor 1 ({})", other),
            },
        }

        current_pos = current_direction.translate_pos(current_pos);
    }

    let mut result = String::from("\n");
    let row_minmax = painted_map.keys().map(|pos| pos.0).minmax();
    let col_minmax = painted_map.keys().map(|pos| pos.1).minmax();
    let (row_min, row_max) = row_minmax.into_option().unwrap();
    let (col_min, col_max) = col_minmax.into_option().unwrap();

    for row in row_min..=row_max {
        for col in col_min..=col_max {
            result.push(
                painted_map
                    .get(&(row, col))
                    .copied()
                    .unwrap_or_default()
                    .into(),
            );
        }

        result.push('\n');
    }

    Ok(result)
}
