use core::fmt;
use std::{
    cmp::Ordering,
    error::Error,
    ops::{Add, AddAssign},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    #[inline]
    const fn cartesian_distance_from_origin(&self) -> u64 {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }

    #[inline]
    const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl fmt::Debug for Vector3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

impl Default for Vector3 {
    #[inline]
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: &Vector3) -> Self::Output {
        self + *rhs
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        *self + *rhs
    }
}

impl Add<Vector3> for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Self::Output {
        *self + rhs
    }
}

impl AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl FromStr for Vector3 {
    type Err = Box<dyn Error>;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y, z)) = s
            .trim_start_matches('<')
            .trim_end_matches('>')
            .split(',')
            .collect_tuple()
        {
            let x = x
                .trim()
                .strip_prefix("x=")
                .ok_or_else(|| format!(r#"{:?} did not start with "x=""#, x))?
                .trim()
                .parse()?;
            let y = y
                .trim()
                .strip_prefix("y=")
                .ok_or_else(|| format!(r#"{:?} did not start with "y=""#, y))?
                .trim()
                .parse()?;
            let z = z
                .trim()
                .strip_prefix("z=")
                .ok_or_else(|| format!(r#"{:?} did not start with "z=""#, z))?
                .trim()
                .parse()?;
            Ok(Self::new(x, y, z))
        } else {
            Err(format!("{:?} could not be split into 3 parts on ','", s).into())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Planet {
    position: Vector3,
    velocity: Vector3,
}

impl Planet {
    #[inline]
    fn speed_influence_x(&mut self, other: &mut Self) {
        let relative = self.position.x.cmp(&other.position.x);
        let to_add = match relative {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => return,
        };

        self.velocity.x += to_add;
        other.velocity.x -= to_add;
    }

    #[inline]
    fn speed_influence_y(&mut self, other: &mut Self) {
        let relative = self.position.y.cmp(&other.position.y);
        let to_add = match relative {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => return,
        };

        self.velocity.y += to_add;
        other.velocity.y -= to_add;
    }

    #[inline]
    fn speed_influence_z(&mut self, other: &mut Self) {
        let relative = self.position.z.cmp(&other.position.z);
        let to_add = match relative {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => return,
        };

        self.velocity.z += to_add;
        other.velocity.z -= to_add;
    }

    #[inline]
    const fn total_energy(&self) -> u64 {
        let potential = self.position.cartesian_distance_from_origin();
        let kinetic = self.velocity.cartesian_distance_from_origin();
        potential * kinetic
    }

    #[inline]
    fn apply_speed_influence(&mut self, other: &mut Self) {
        // println!("I: {:?} -- {:?}", self, other);
        self.speed_influence_x(other);
        self.speed_influence_y(other);
        self.speed_influence_z(other);
        // println!("O: {:?} -- {:?}\n", self, other);
    }

    #[inline]
    fn apply_velocity(&mut self) {
        // println!("position: {:?}, velocity: {:?}", self.position, self.velocity);
        let velocity = self.velocity;
        self.position += velocity;
        // println!("    new : {:?}", self.position);
    }
}

impl From<Vector3> for Planet {
    #[inline]
    fn from(value: Vector3) -> Self {
        Self {
            position: value,
            velocity: Vector3::default(),
        }
    }
}

#[inline]
fn parse_into_planets(input: &str) -> Result<Vec<Planet>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| Vector3::from_str(line))
        .map_ok(|position| position.into())
        .collect()
}

#[inline]
fn energy_after_steps(system: &mut [Planet], steps: usize) -> u64 {
    for _step in 0..steps {
        // println!("\nStep {}:", _step);
        for i in 0..system.len() {
            for j in (i + 1)..system.len() {
                let (first, second) = system.split_at_mut(j);
                first[i].apply_speed_influence(&mut second[0]);
            }
        }

        system.iter_mut().for_each(Planet::apply_velocity);
    }

    system.iter().map(Planet::total_energy).sum()
}

#[aoc(day12, part1)]
fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut planets = parse_into_planets(input)?;
    Ok(energy_after_steps(&mut planets, 1000))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    const EXAMPLE2: &str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    #[test]
    fn part1_example1() {
        let mut planets = parse_into_planets(EXAMPLE1).unwrap();
        assert_eq!(energy_after_steps(&mut planets, 10), 179);
    }

    #[test]
    fn part1_example2() {
        let mut planets = parse_into_planets(EXAMPLE2).unwrap();
        assert_eq!(energy_after_steps(&mut planets, 100), 1940);
    }
}
