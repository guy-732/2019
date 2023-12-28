use core::fmt;
use std::{
    iter,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
struct Layer {
    width: usize,
    content: Box<[u8]>,
}

impl Layer {
    #[inline]
    fn from_iterator<I: Iterator<Item = u8>>(
        iter: &mut I,
        width: usize,
        height: usize,
    ) -> Option<Self> {
        let mut final_vec = Vec::with_capacity(height);
        for _ in 0..(height * width) {
            final_vec.push(iter.next()?.checked_sub(b'0')?);
        }

        Some(Self {
            width,
            content: final_vec.into_boxed_slice(),
        })
    }

    #[inline]
    const fn get_height(&self) -> usize {
        self.content.len() / self.get_width()
    }

    #[inline]
    const fn get_width(&self) -> usize {
        self.width
    }

    #[inline]
    fn get(&self, row: usize, col: usize) -> Option<&u8> {
        self.content.get(row * self.get_width() + col)
    }

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut u8> {
        self.content.get_mut(row * self.get_width() + col)
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = &u8> {
        self.content.iter()
    }
}

impl Index<(usize, usize)> for Layer {
    type Output = u8;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).unwrap_or_else(|| {
            panic!(
                "Index {:?} does not exist (height: {}, width: {})",
                index,
                self.get_height(),
                self.get_width()
            )
        })
    }
}

impl IndexMut<(usize, usize)> for Layer {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let height = self.get_height();
        let width = self.get_width();

        self.get_mut(index.0, index.1).unwrap_or_else(|| {
            panic!(
                "Index {:?} does not exist (height: {}, width: {})",
                index, height, width
            )
        })
    }
}

impl fmt::Display for Layer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.get_height() {
            for col in 0..self.get_width() {
                write!(
                    f,
                    "{}",
                    match self[(row, col)] {
                        0 => ' ',
                        1 => '#',
                        other => (b'0' + other) as char,
                    }
                )?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[inline]
fn parse_with(input: &[u8], width: usize, height: usize) -> Vec<Layer> {
    let mut iter = input.iter().copied();
    let mut result = vec![];
    while let Some(layer) = Layer::from_iterator(&mut iter, width, height) {
        result.push(layer);
    }

    result
}

#[aoc_generator(day08)]
fn parse(input: &[u8]) -> Vec<Layer> {
    parse_with(input, 25, 6)
}

#[aoc(day08, part1)]
fn part1(layers: &[Layer]) -> usize {
    // for (i, layer) in (1..).zip(layers) {
    //     println!("Layer {}:", i);
    //     println!("{}", layer);
    // }

    let less_0 = layers
        .iter()
        .min_by_key(|&layer| layer.iter().filter(|&&n| n == 0).count())
        .unwrap();

    less_0.iter().filter(|&&n| n == 1).count() * less_0.iter().filter(|&&n| n == 2).count()
}

#[aoc(day08, part2)]
fn part2(layers: &[Layer]) -> String {
    let width = layers[0].get_width();
    let height = layers[0].get_height();

    let mut final_layer = Layer::from_iterator(&mut iter::repeat(b'0'), width, height).unwrap();
    for row in 0..height {
        for col in 0..width {
            let target = layers
                .iter()
                .find_map(|layer| {
                    let color = layer[(row, col)];
                    (color != 2).then_some(color)
                })
                .unwrap_or(2);

            final_layer[(row, col)] = target;
        }
    }

    format!("\n{}", final_layer)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLE: &[u8] = b"123456789012\n";
    const PART2_EXAMPLE: &[u8] = b"0222112222120000\n";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_with(PART1_EXAMPLE, 3, 2)), 1);
    }

    #[test]
    fn part2_example() {
        assert_eq!(&part2(&parse_with(PART2_EXAMPLE, 2, 2)), "\n #\n# \n");
    }
}
