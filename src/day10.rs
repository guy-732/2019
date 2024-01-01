use std::{cmp, collections::BinaryHeap, iter};

use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use num::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(isize, isize);

impl Pos {
    /// returns distance from self to other
    #[inline]
    fn distance_to(&self, other: &Self) -> Self {
        Self(other.0 - self.0, other.1 - self.1)
    }

    #[inline]
    fn reduced_vector(&self) -> Self {
        let gcd = self.0.gcd(&self.1);
        Self(self.0 / gcd, self.1 / gcd)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Asteroid,
}

impl From<char> for Tile {
    #[inline]
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Asteroid,
            other => panic!("{:?} was neither '.' nor '#'", other),
        }
    }
}

#[inline]
fn count_seen_asteroids(pos: Pos, grid: &[Box<[Tile]>]) -> usize {
    let mut seeing_set = FnvHashSet::default();

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, tile) in row.iter().enumerate() {
            let tile_pos = Pos(row_index as isize, col_index as isize);
            if pos == tile_pos || !matches!(tile, Tile::Asteroid) {
                continue;
            }

            seeing_set.insert(pos.distance_to(&tile_pos).reduced_vector());
        }
    }

    seeing_set.len()
}

#[inline]
fn make_laser_hit_map(
    pos: Pos,
    grid: &[Box<[Tile]>],
) -> FnvHashMap<Pos, BinaryHeap<(cmp::Reverse<usize>, Pos)>> {
    let mut result: FnvHashMap<Pos, BinaryHeap<(cmp::Reverse<usize>, Pos)>> = FnvHashMap::default();

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, tile) in row.iter().enumerate() {
            let tile_pos = Pos(row_index as isize, col_index as isize);
            if pos == tile_pos || !matches!(tile, Tile::Asteroid) {
                continue;
            }

            let distance = pos.distance_to(&tile_pos);
            let gcd = distance.0.gcd(&distance.1);
            let reduced_dist = Pos(distance.0 / gcd, distance.1 / gcd);
            if let Some(heap) = result.get_mut(&reduced_dist) {
                heap.push((cmp::Reverse(gcd as usize), tile_pos));
            } else {
                result.insert(
                    reduced_dist,
                    iter::once((cmp::Reverse(gcd as usize), tile_pos)).collect()
                );
            }
        }
    }

    result
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Box<[Tile]>> {
    input
        .lines()
        .map(|line| line.chars().map_into().collect())
        .collect_vec()
}

#[aoc(day10, part1)]
fn part1(grid: &[Box<[Tile]>]) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter().enumerate().filter_map(move |(col_index, tile)| {
                if matches!(tile, Tile::Asteroid) {
                    Some(count_seen_asteroids(
                        Pos(row_index as isize, col_index as isize),
                        grid,
                    ))
                } else {
                    None
                }
            })
        })
        .max()
        .unwrap()
}

#[aoc(day10, part2)]
fn part2(grid: &[Box<[Tile]>]) -> usize {
    let laser_pos = grid
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter().enumerate().filter_map(move |(col_index, tile)| {
                if matches!(tile, Tile::Asteroid) {
                    let tile_pos = Pos(row_index as isize, col_index as isize);
                    Some((tile_pos, count_seen_asteroids(tile_pos, grid)))
                } else {
                    None
                }
            })
        })
        .max_by_key(|key| key.1)
        .unwrap()
        .0;

    // println!("Laser pos: {:?}", laser_pos);

    let laser_hit = make_laser_hit_map(laser_pos, grid);
    // println!("{:#?}", laser_hit);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

    const EXAMPLE2: &str = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

    const EXAMPLE3: &str = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

    const EXAMPLE4: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse(EXAMPLE1)), 33);
        assert_eq!(part1(&parse(EXAMPLE2)), 35);
        assert_eq!(part1(&parse(EXAMPLE3)), 41);
        assert_eq!(part1(&parse(EXAMPLE4)), 210);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE4)), 802);
    }
}
