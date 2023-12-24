use itertools::Itertools;
use std::num::ParseIntError;

#[inline]
fn parse_into_nums<'s>(input: &'s str) -> impl Iterator<Item = Result<u64, ParseIntError>> + 's {
    input.lines().map(|line| line.parse())
}

#[inline]
fn calculate_fuel_load(weight: u64) -> u64 {
    (weight / 3).checked_sub(2).unwrap_or(0)
}

#[inline]
fn calculate_total_fuel_load_for_fuel(mut fuel_weight: u64) -> u64 {
    let mut sum = fuel_weight;
    loop {
        if fuel_weight == 0 {
            break sum;
        }

        fuel_weight = calculate_fuel_load(fuel_weight);
        sum += fuel_weight;
    }
}

#[aoc(day01, part1)]
fn part1(input: &str) -> Result<u64, ParseIntError> {
    parse_into_nums(input).map_ok(calculate_fuel_load).sum()
}

#[aoc(day01, part2)]
fn part2(input: &str) -> Result<u64, ParseIntError> {
    parse_into_nums(input)
        .map_ok(calculate_fuel_load)
        .map_ok(calculate_total_fuel_load_for_fuel)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("12").unwrap(), 2);
        assert_eq!(part1("14").unwrap(), 2);
        assert_eq!(part1("1969").unwrap(), 654);
        assert_eq!(part1("100756").unwrap(), 33583);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("14").unwrap(), 2);
        assert_eq!(part2("1969").unwrap(), 966);
        assert_eq!(part2("100756").unwrap(), 50346);
    }
}
