use std::collections::HashMap;

use itertools::Itertools;

fn does_towel_fit<'a>(
    design: &'a str,
    towels: &[&str],
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if design.is_empty() {
        true
    } else {
        if cache.contains_key(design) {
            return *cache.get(design).unwrap();
        }
        let res = towels.iter().any(|towel| {
            design.starts_with(towel) && does_towel_fit(&design[towel.len()..], towels, cache)
        });
        cache.insert(design, res);
        res
    }
}

#[aoc(day19, part1)]
fn part1(input: &str) -> u64 {
    let mut lines_iter = input.lines();
    let mut cache = HashMap::new();

    let towels = lines_iter.next().unwrap().split(", ").collect_vec();
    lines_iter.next().unwrap();

    lines_iter
        .filter(|line| does_towel_fit(line, &towels, &mut cache))
        .count() as u64
}

fn count_arrangements<'a>(
    design: &'a str,
    towels: &[&str],
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        1
    } else {
        if cache.contains_key(design) {
            return *cache.get(design).unwrap();
        }
        let res = towels
            .iter()
            .filter(|&towel| design.starts_with(towel))
            .map(|towel| count_arrangements(&design[towel.len()..], towels, cache))
            .sum();
        cache.insert(design, res);
        res
    }
}

#[aoc(day19, part2)]
fn part2(input: &str) -> u64 {
    let mut lines_iter = input.lines();
    let mut cache = HashMap::new();

    let towels = lines_iter.next().unwrap().split(", ").collect_vec();
    lines_iter.next().unwrap();

    lines_iter
        .map(|line| count_arrangements(line, &towels, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 16);
    }
}
