use std::{
    borrow::Borrow,
    cmp::Ordering,
    error::Error,
    fmt,
    io::{stdout, IsTerminal},
    thread,
    time::Duration,
};

use fnv::FnvHashMap;
use intcode_vm::{memory::Memory, IntcodeVM, VMResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Left,
    StayStill,
    Right,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Move {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "move left",
            Self::Right => "move right",
            Self::StayStill => "not moving",
        }
    }
}

impl From<Move> for i64 {
    fn from(value: Move) -> Self {
        match value {
            Move::Left => -1,
            Move::StayStill => 0,
            Move::Right => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TileId {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

impl From<&TileId> for char {
    fn from(value: &TileId) -> Self {
        match value {
            TileId::Empty => ' ',
            TileId::Wall => '#',
            TileId::Block => '⯀',
            TileId::HorizontalPaddle => '―',
            TileId::Ball => 'O',
        }
    }
}

impl From<TileId> for char {
    fn from(value: TileId) -> Self {
        value.borrow().into()
    }
}

impl TryFrom<i64> for TileId {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            4 => Self::Ball,
            other => {
                return Err(format!(
                    "tile_id must be one of [0, 1, 2, 3, 4]: was {}",
                    other
                ))
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TilePosition {
    x: i64,
    y: i64,
}

impl TilePosition {
    fn new(x: i64, y: i64) -> Self {
        assert!(x >= 0, "x cannot be negative ({})", x);
        assert!(y >= 0, "y cannot be negative ({})", y);
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DecodeDraw {
    Success(TilePosition, TileId),
    ScoreDisplay(i64),
    WaitingForInput,
    Halt,
}

#[inline]
fn decode_draw(vm: &mut IntcodeVM<i64>) -> Result<DecodeDraw, Box<dyn Error>> {
    let x = match vm.run()? {
        VMResult::Output(v) => v,
        VMResult::WaitingForInput => return Ok(DecodeDraw::WaitingForInput),
        VMResult::Halted => return Ok(DecodeDraw::Halt),
    };

    let y = match vm.run()? {
        VMResult::Output(v) => v,
        VMResult::WaitingForInput => return Ok(DecodeDraw::WaitingForInput),
        VMResult::Halted => return Ok(DecodeDraw::Halt),
    };

    let id = match vm.run()? {
        VMResult::Output(v) => v,
        VMResult::WaitingForInput => return Ok(DecodeDraw::WaitingForInput),
        VMResult::Halted => return Ok(DecodeDraw::Halt),
    };

    Ok(if x == -1 && y == 0 {
        DecodeDraw::ScoreDisplay(id)
    } else {
        DecodeDraw::Success(TilePosition::new(x, y), id.try_into()?)
    })
}

#[aoc(day13, part1)]
fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut vm = input.parse::<IntcodeVM<_>>()?;
    let mut tiles = FnvHashMap::default();

    loop {
        match decode_draw(&mut vm)? {
            DecodeDraw::Success(pos, id) => tiles.insert(pos, id),
            DecodeDraw::WaitingForInput => panic!("Sould not be waiting for an input"),
            DecodeDraw::ScoreDisplay(_) => panic!("Score display should not show up"),
            DecodeDraw::Halt => {
                return Ok(tiles
                    .values()
                    .filter(|id| matches!(id, TileId::Block))
                    .count())
            }
        };
    }
}

const CLEAR_TERM: &str = "\x1b[H\x1b[2J\x1b[3J";

#[aoc(day13, part2)]
fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    #[allow(unused_variables)]
    let is_terminal = stdout().is_terminal();
    let is_terminal = false; // override
    let mut vm = input.parse::<Memory<_>>()?;
    vm.set(0, 2);

    let mut vm = vm.into();
    let mut tiles = FnvHashMap::default();
    let mut score = 0;
    let mut last_ball_pos_x = 0;
    let mut last_paddle_pos_x = 0;

    loop {
        loop {
            match decode_draw(&mut vm)? {
                DecodeDraw::Success(pos, id) => {
                    tiles.insert(pos, id);
                    if id == TileId::Ball {
                        last_ball_pos_x = pos.x;
                    } else if id == TileId::HorizontalPaddle {
                        last_paddle_pos_x = pos.x;
                    }
                }
                DecodeDraw::WaitingForInput => break,
                DecodeDraw::ScoreDisplay(v) => {
                    score = v;
                }
                DecodeDraw::Halt => {
                    // println!("Score: {}", score);
                    // println!("{}", draw(&tiles));
                    return Ok(score);
                }
            };
        }

        let mv = match last_ball_pos_x.cmp(&last_paddle_pos_x) {
            Ordering::Equal => Move::StayStill,
            Ordering::Greater => Move::Right,
            Ordering::Less => Move::Left,
        };
        vm.set_next_input(mv.into());

        if is_terminal {
            let drawn = draw(&tiles);
            println!(
                "{CLEAR_TERM}Score: {}\n{}\nHighly sofisticated movement choser: {}",
                score, drawn, mv
            );
            thread::sleep(Duration::from_millis(25));
        }
    }
}

fn draw(tiles: &FnvHashMap<TilePosition, TileId>) -> String {
    let max_x = tiles.keys().map(|pos| pos.x).max().expect("tiles is empty");
    let max_y = tiles
        .keys()
        .map(|pos| pos.y)
        .max()
        .expect("tiles is empty AND we already iterated over values... that DID exist");

    let mut drawn = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = TilePosition { x, y };
            drawn.push(tiles.get(&pos).unwrap_or(&TileId::Empty).into());
        }

        drawn.push('\n');
    }

    drawn
}
