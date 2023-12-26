use core::slice;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    #[inline]
    const fn distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    #[inline]
    const fn distance_from(&self, other: &Point) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Segment {
    segment_ends: (Point, Point),
}

impl Segment {
    #[inline]
    const fn min_x(&self) -> i64 {
        let x1 = self.segment_ends.0.x;
        let x2 = self.segment_ends.1.x;

        if x1 < x2 {
            x1
        } else {
            x2
        }
    }

    #[inline]
    const fn max_x(&self) -> i64 {
        let x1 = self.segment_ends.0.x;
        let x2 = self.segment_ends.1.x;

        if x1 > x2 {
            x1
        } else {
            x2
        }
    }

    #[inline]
    const fn min_y(&self) -> i64 {
        let y1 = self.segment_ends.0.y;
        let y2 = self.segment_ends.1.y;

        if y1 < y2 {
            y1
        } else {
            y2
        }
    }

    #[inline]
    const fn max_y(&self) -> i64 {
        let y1 = self.segment_ends.0.y;
        let y2 = self.segment_ends.1.y;

        if y1 > y2 {
            y1
        } else {
            y2
        }
    }

    #[inline]
    const fn any_x(&self) -> i64 {
        self.segment_ends.0.x
    }

    #[inline]
    const fn any_y(&self) -> i64 {
        self.segment_ends.0.y
    }

    #[inline]
    const fn is_vertical(&self) -> bool {
        self.segment_ends.0.x == self.segment_ends.1.x
    }

    #[inline]
    const fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }

    #[inline]
    const fn intersection_point(&self, other: &Segment) -> Option<Point> {
        if self.is_vertical() {
            if other.is_vertical() {
                None
            } else if self.min_y() <= other.any_y()
                && other.any_y() <= self.max_y()
                && other.min_x() <= self.any_x()
                && self.any_x() <= other.max_x()
            {
                Some(Point {
                    x: self.any_x(),
                    y: other.any_y(),
                })
            } else {
                None
            }
        } else if other.is_horizontal() {
            None
        } else if self.min_x() <= other.any_x()
            && other.any_x() <= self.max_x()
            && other.min_y() <= self.any_y()
            && self.any_y() <= other.max_y()
        {
            Some(Point {
                x: other.any_x(),
                y: self.any_y(),
            })
        } else {
            None
        }
    }

    #[inline]
    const fn length(&self) -> u64 {
        self.segment_ends.0.distance_from(&self.segment_ends.1)
    }
}

impl From<(Point, Point)> for Segment {
    fn from(value: (Point, Point)) -> Self {
        Self {
            segment_ends: value,
        }
    }
}

#[aoc_generator(day03)]
fn parse(input: &str) -> Result<(Vec<Segment>, Vec<Segment>), &'static str> {
    input
        .lines()
        .map(|line| {
            let mut result = vec![];
            let mut previous_point = Point { x: 0, y: 0 };

            for part in line.split(',').map(|part| part.trim()) {
                let new_point = if let Some(num) = part.strip_prefix('U') {
                    Point {
                        y: previous_point.y + num.parse::<i64>().unwrap(),
                        x: previous_point.x,
                    }
                } else if let Some(num) = part.strip_prefix('D') {
                    Point {
                        y: previous_point.y - num.parse::<i64>().unwrap(),
                        x: previous_point.x,
                    }
                } else if let Some(num) = part.strip_prefix('L') {
                    Point {
                        y: previous_point.y,
                        x: previous_point.x - num.parse::<i64>().unwrap(),
                    }
                } else if let Some(num) = part.strip_prefix('R') {
                    Point {
                        y: previous_point.y,
                        x: previous_point.x + num.parse::<i64>().unwrap(),
                    }
                } else {
                    panic!("{:?} did not start with any of ['U', 'D', 'L', 'R']", part);
                };

                result.push((previous_point, new_point).into());
                previous_point = new_point;
            }

            result
        })
        .collect_tuple()
        .ok_or("Only 2 lines expected, found at least 3")
}

#[aoc(day03, part1)]
fn part1(wires: &(Vec<Segment>, Vec<Segment>)) -> i64 {
    wires
        .0
        .iter()
        .flat_map(|a_segment| {
            wires
                .1
                .iter()
                .filter_map(|b_segment| b_segment.intersection_point(a_segment))
        })
        // .inspect(|pts| eprintln!("{:?}", pts))
        .filter_map(|point| {
            if point.x == 0 && point.y == 0 {
                None
            } else {
                Some(point.distance())
            }
        })
        .min()
        .unwrap()
}

#[derive(Debug, Clone)]
struct SegmentSliceIterator<'s> {
    slice_iter: slice::Iter<'s, Segment>,
    current_length: u64,
}

impl<'s> SegmentSliceIterator<'s> {
    #[inline]
    fn new(slice_iter: slice::Iter<'s, Segment>) -> Self {
        Self {
            slice_iter,
            current_length: 0,
        }
    }
}

impl<'s> Iterator for SegmentSliceIterator<'s> {
    type Item = (u64, &'s Segment);

    fn next(&mut self) -> Option<Self::Item> {
        let length = self.current_length;
        let element = self.slice_iter.next()?;
        self.current_length += element.length();
        Some((length, element))
    }
}

#[aoc(day03, part2)]
fn part2(wires: &(Vec<Segment>, Vec<Segment>)) -> u64 {
    let wire_a = wires.0.as_slice();
    let wire_b = wires.1.as_slice();

    SegmentSliceIterator::new(wire_a.iter())
        .flat_map(|(a_length, a_segment)| {
            SegmentSliceIterator::new(wire_b.iter()).filter_map(move |(b_length, b_segment)| {
                let result = b_segment.intersection_point(a_segment)?;
                Some((
                    a_length,
                    b_length,
                    result,
                    &a_segment.segment_ends.0,
                    &b_segment.segment_ends.0,
                ))
            })
        })
        .filter_map(|(a_length, b_length, point, a_pts, b_pts)| {
            if point.x == 0 && point.y == 0 {
                None
            } else {
                Some(a_length + b_length + point.distance_from(a_pts) + point.distance_from(b_pts))
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"R8,U5,L5,D3
U7,R6,D4,L4"#;

    const EXAMPLE2: &str = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;

    const EXAMPLE3: &str = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse(EXAMPLE1).unwrap()), 6);
        assert_eq!(part1(&parse(EXAMPLE2).unwrap()), 159);
        assert_eq!(part1(&parse(EXAMPLE3).unwrap()), 135);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&parse(EXAMPLE1).unwrap()), 30);
        assert_eq!(part2(&parse(EXAMPLE2).unwrap()), 610);
        assert_eq!(part2(&parse(EXAMPLE3).unwrap()), 410);
    }
}
