use std::{collections::VecDeque, error::Error, iter};

use fnv::{FnvHashMap, FnvHashSet};

#[derive(Debug, Clone, Default)]
struct Graph {
    adj_list: FnvHashMap<String, FnvHashSet<String>>,
}

impl Graph {
    #[inline]
    fn calculate_part1(&self) -> u64 {
        let mut queue = VecDeque::from([("COM", 0)]);
        let mut count = 0;
        while let Some((key, depth)) = queue.pop_front() {
            count += depth;

            if let Some(adj) = self.adj_list.get(key) {
                queue.extend(adj.iter().map(|key| (key.as_str(), depth + 1)));
            }
        }

        count
    }

    #[inline]
    fn calculate_distance_between(&self, start: &str, end: &str) -> u64 {
        let mut queue = VecDeque::from([(start, 0)]);
        let mut visited = FnvHashSet::from_iter([start]);
        while let Some((key, depth)) = queue.pop_front() {
            if key == end {
                return depth;
            }

            for key in &self.adj_list[key] {
                if visited.insert(key) {
                    queue.push_back((&key, depth + 1));
                }
            }
        }

        unreachable!("No path");
    }
}

#[aoc_generator(day06, part1)]
fn parse_part1(input: &str) -> Result<Graph, Box<dyn Error>> {
    let mut graph = Graph::default();

    for line in input.lines() {
        let (before, after) = line
            .split_once(')')
            .ok_or_else(|| format!("Could not split {:?} on ')'", line))?;

        let before = before.trim();
        if let Some(list) = graph.adj_list.get_mut(before) {
            list.insert(after.trim().to_owned());
        } else {
            graph.adj_list.insert(
                before.to_owned(),
                iter::once(after.trim().to_owned()).collect(),
            );
        }
    }

    Ok(graph)
}

#[aoc(day06, part1)]
fn part1(graph: &Graph) -> u64 {
    graph.calculate_part1()
}

#[aoc_generator(day06, part2)]
fn parse_part2(input: &str) -> Result<Graph, Box<dyn Error>> {
    let mut graph = Graph::default();

    for line in input.lines() {
        let (before, after) = line
            .split_once(')')
            .ok_or_else(|| format!("Could not split {:?} on ')'", line))?;

        let before = before.trim();
        let after = after.trim();
        if let Some(list) = graph.adj_list.get_mut(before) {
            list.insert(after.to_owned());
        } else {
            graph
                .adj_list
                .insert(before.to_owned(), iter::once(after.to_owned()).collect());
        }

        if let Some(list) = graph.adj_list.get_mut(after) {
            list.insert(before.to_owned());
        } else {
            graph
                .adj_list
                .insert(after.to_owned(), iter::once(before.to_owned()).collect());
        }
    }

    Ok(graph)
}

#[aoc(day06, part2)]
fn part2(graph: &Graph) -> u64 {
    graph.calculate_distance_between("YOU", "SAN") - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_P1: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;

    const EXAMPLE_P2: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE_P1).unwrap()), 42);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE_P2).unwrap()), 4);
    }
}
