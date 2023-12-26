use std::{error::Error, ops::RangeInclusive};

#[inline]
fn is_valid_num_part1(mut num: u32) -> bool {
    let mut last = 10; // not a digit and greater than all digits, won't cause false positive
    let mut has_repeat = false;
    for _ in 0..6 {
        let digit = num % 10;
        num /= 10;

        match digit.cmp(&last) {
            std::cmp::Ordering::Equal => has_repeat = true,
            std::cmp::Ordering::Greater => return false, // not in decreasing order
            _ => (),
        }

        last = digit;
    }

    has_repeat
}

#[inline]
fn is_valid_num_part2(mut num: u32) -> bool {
    let mut last = num % 10;
    num /= 10;
    let mut has_repeat = false;
    let mut repeat_count = 0;
    for _ in 0..5 {
        let digit = num % 10;
        num /= 10;

        if digit == last {
            repeat_count += 1;
        } else {
            if repeat_count == 1 {
                has_repeat = true;
            }

            repeat_count = 0;
            if digit > last {
                return false; // not in decreasing order
            }
        }

        last = digit;
    }

    has_repeat || repeat_count == 1
}

#[aoc_generator(day04, part1, brute)]
fn parse_p1_brute(input: &str) -> Result<RangeInclusive<u32>, Box<dyn Error>> {
    parse_as_range(input)
}

#[aoc_generator(day04, part2, brute)]
fn parse_p2_brute(input: &str) -> Result<RangeInclusive<u32>, Box<dyn Error>> {
    parse_as_range(input)
}

#[inline]
fn parse_as_range(input: &str) -> Result<RangeInclusive<u32>, Box<dyn Error>> {
    let (left, right) = input
        .split_once('-')
        .ok_or_else(|| format!("Could not split {:?} on '-'", input))?;
    Ok(left.parse()?..=right.parse()?)
}

#[aoc(day04, part1, brute)]
fn part1_brute(range: &RangeInclusive<u32>) -> usize {
    range.clone().filter(|&num| is_valid_num_part1(num)).count()
}

#[aoc(day04, part2, brute)]
fn part2_brute(range: &RangeInclusive<u32>) -> usize {
    range
        .clone()
        .filter(|&num| is_valid_num_part2(num))
        // .inspect(|&num| println!("{}", num))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_num_validity() {
        assert!(is_valid_num_part1(111111));
        assert!(!is_valid_num_part1(223450));
        assert!(!is_valid_num_part1(123789));
    }

    #[test]
    fn test_part2_num_validity() {
        assert!(is_valid_num_part2(112233));
        assert!(!is_valid_num_part2(123444));
        assert!(is_valid_num_part2(111122));
        assert!(!is_valid_num_part2(223450));
        assert!(!is_valid_num_part2(122234));
        assert!(is_valid_num_part2(112333));
    }
}
