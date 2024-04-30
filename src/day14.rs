use std::{error::Error, num::NonZeroU64, str::FromStr};

use fnv::FnvHashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Chemical {
    name: String,
    amount: NonZeroU64,
}

impl FromStr for Chemical {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((amount, name)) = s.trim().split_once(' ') else {
            return Err(format!("Could not split {:?} on ' '", s.trim()).into());
        };

        Ok(Self {
            name: name.to_owned(),
            amount: amount.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Recipe {
    input: Vec<Chemical>,
    result: Chemical,
}

impl FromStr for Recipe {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((inputs, result)) = s.trim().split_once("=>") else {
            return Err(format!(r#"Could not split {:?} on "=>""#, s.trim()).into());
        };

        Ok(Self {
            input: inputs.split(',').map(|chem| chem.parse()).try_collect()?,
            result: result.parse()?,
        })
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Result<FnvHashMap<String, Recipe>, Box<dyn Error>> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line.parse::<Recipe>())
            }
        })
        .map_ok(|recipe| (recipe.result.name.clone(), recipe))
        .try_collect()
}

#[aoc(day14, part1)]
fn part1(recipes: &FnvHashMap<String, Recipe>) -> u64 {
    let mut ore_required = 0;
    let mut leftovers: FnvHashMap<&str, NonZeroU64> = FnvHashMap::default();
    let mut requires = vec![("FUEL", NonZeroU64::new(1).unwrap())];

    while let Some((chem, amount)) = requires.pop() {
        if chem == "ORE" {
            ore_required += amount.get();
            continue;
        }

        if let Some(remains) = leftovers.remove(chem) {
            if remains > amount {
                leftovers.insert(chem, NonZeroU64::new(remains.get() - amount.get()).unwrap());
            } else if let Some(needs) = NonZeroU64::new(amount.get() - remains.get()) {
                requires.push((chem, needs));
            }

            continue;
        }

        let recipe = recipes
            .get(chem)
            .unwrap_or_else(|| panic!("Chemical {:?} is not in recipe list", chem));

        let repeat =
            if let Some(reaction_leftover) = NonZeroU64::new(amount.get() % recipe.result.amount) {
                leftovers.insert(
                    chem,
                    NonZeroU64::new(recipe.result.amount.get() - reaction_leftover.get())
                        .expect("literally impossible"),
                );
                1
            } else {
                0
            } + (amount.get() / recipe.result.amount);

        requires.extend(recipe.input.iter().map(|input| {
            (
                input.name.as_str(),
                NonZeroU64::new(input.amount.get() * repeat).unwrap(),
            )
        }));
    }

    ore_required
}

#[aoc(day14, part2)]
fn part2(recipes: &FnvHashMap<String, Recipe>) -> u64 {
    const INITIAL_ORE: u64 = 1000000000000_u64;
    let mut ore_remaining = INITIAL_ORE;
    let mut leftovers: FnvHashMap<&str, NonZeroU64> = FnvHashMap::default();
    let mut requires = vec![];
    let mut fuel_produced = 0;
    let mut has_done_leftovers_check = false;

    loop {
        requires.push(("FUEL", NonZeroU64::new(1).unwrap()));

        while let Some((chem, amount)) = requires.pop() {
            if chem == "ORE" {
                if let Some(remain) = ore_remaining.checked_sub(amount.get()) {
                    ore_remaining = remain;
                    continue;
                }

                break;
            }

            if let Some(remains) = leftovers.remove(chem) {
                if remains > amount {
                    leftovers.insert(chem, NonZeroU64::new(remains.get() - amount.get()).unwrap());
                } else if let Some(needs) = NonZeroU64::new(amount.get() - remains.get()) {
                    requires.push((chem, needs));
                }

                continue;
            }

            let recipe = recipes
                .get(chem)
                .unwrap_or_else(|| panic!("Chemical {:?} is not in recipe list", chem));

            let repeat = if let Some(reaction_leftover) =
                NonZeroU64::new(amount.get() % recipe.result.amount)
            {
                leftovers.insert(
                    chem,
                    NonZeroU64::new(recipe.result.amount.get() - reaction_leftover.get())
                        .expect("literally impossible"),
                );
                1
            } else {
                0
            } + (amount.get() / recipe.result.amount);

            requires.extend(recipe.input.iter().map(|input| {
                (
                    input.name.as_str(),
                    NonZeroU64::new(input.amount.get() * repeat).unwrap(),
                )
            }));
        }

        if requires.is_empty() {
            fuel_produced += 1;

            if !has_done_leftovers_check && leftovers.is_empty() {
                has_done_leftovers_check = true;

                let ore_per_batch = INITIAL_ORE - ore_remaining;
                let extra_full_batches = ore_remaining / ore_per_batch;
                fuel_produced += extra_full_batches * fuel_produced;
                ore_remaining -= extra_full_batches * ore_per_batch;
            }
        } else {
            return fuel_produced;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    const EXAMPLE2: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    const EXAMPLE3: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const EXAMPLE4: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    const EXAMPLE5: &str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(EXAMPLE1).unwrap()), 31);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE2).unwrap()), 165);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(&parse(EXAMPLE3).unwrap()), 13312);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1(&parse(EXAMPLE4).unwrap()), 180697);
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part1(&parse(EXAMPLE5).unwrap()), 2210736);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(EXAMPLE3).unwrap()), 82892753);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(EXAMPLE4).unwrap()), 5586022);
    }

    #[test]
    #[ignore = "Takes too much time"]
    fn part2_example3() {
        assert_eq!(part2(&parse(EXAMPLE5).unwrap()), 460664);
    }
}
