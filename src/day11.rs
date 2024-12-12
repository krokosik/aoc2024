use std::{collections::HashMap, iter::once};

const N1: usize = 25;

#[aoc(day11, part1, fold)]
fn part1(input: &str) -> usize {
    let iter: Box<dyn Iterator<Item = usize>> = Box::new(
        input
            .lines()
            .flat_map(|line| line.split_whitespace())
            .map(|stone_number| stone_number.parse::<usize>().unwrap()),
    );

    (0..N1)
        .fold(iter, |iter, _| Box::new(part1_step(iter)))
        .count()
}

fn part1_step(iter: impl Iterator<Item = usize>) -> impl Iterator<Item = usize> {
    iter.flat_map(|stone_number| match stone_number {
        0 => once(Some(1)).chain(once(None)),
        stone => {
            let n_digits = (stone as f64).log10().floor() as usize + 1;
            if n_digits % 2 == 1 {
                once(Some(stone * 2024)).chain(once(None))
            } else {
                once(Some(stone / 10usize.pow((n_digits / 2) as u32)))
                    .chain(once(Some(stone % 10usize.pow((n_digits / 2) as u32))))
            }
        }
    })
    .flatten()
}

#[aoc(day11, part1, recursive)]
fn part1_alternative(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(|stone_number| stone_number.parse::<usize>().unwrap())
        .map(|stone| part1_process_stone(stone, 25))
        .sum()
}

fn part1_process_stone(stone: usize, n_steps: usize) -> usize {
    if n_steps == 0 {
        1
    } else {
        let n = n_steps - 1;
        if stone == 0 {
            part1_process_stone(1, n)
        } else {
            let n_digits = (stone as f64).log10().floor() as usize + 1;

            if n_digits % 2 == 1 {
                part1_process_stone(stone * 2024, n)
            } else {
                let n_half = n_digits / 2;
                let first_half = stone / 10usize.pow(n_half as u32);
                let second_half = stone % 10usize.pow(n_half as u32);
                part1_process_stone(first_half, n) + part1_process_stone(second_half, n)
            }
        }
    }
}
const N2: usize = 75;

type Cache = HashMap<[usize; 2], usize>;

#[aoc(day11, part1, tree)]
fn tree_solution(input: &str) -> usize {
    tree_cached_solution(input, N1)
}

#[aoc(day11, part2, tree)]
fn part2(input: &str) -> usize {
    tree_cached_solution(input, N2)
}

fn tree_cached_solution(input: &str, n: usize) -> usize {
    let mut cache: Cache = HashMap::new();

    input
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(|stone_number| stone_number.parse::<usize>().unwrap())
        .map(|stone| part2_process_stone(stone, n, &mut cache))
        .sum()
}

fn part2_process_stone(stone: usize, n: usize, cache: &mut Cache) -> usize {
    let key = [stone, n];
    if !cache.contains_key(&[stone, n]) {
        if n == 0 {
            cache.insert(key, 1);
        } else {
            let n = n - 1;
            if stone == 0 {
                let result = part2_process_stone(1, n, cache);
                cache.insert(key, result);
            } else {
                let n_digits = (stone as f64).log10().floor() as usize + 1;

                if n_digits % 2 == 1 {
                    let result = part2_process_stone(stone * 2024, n, cache);
                    cache.insert(key, result);
                } else {
                    let n_half = n_digits / 2;
                    let first_half = stone / 10usize.pow(n_half as u32);
                    let second_half = stone % 10usize.pow(n_half as u32);
                    let first_half_result = part2_process_stone(first_half, n, cache);
                    let second_half_result = part2_process_stone(second_half, n, cache);
                    cache.insert(key, first_half_result + second_half_result);
                }
            }
        }
    }
    cache[&key]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(tree_cached_solution(EXAMPLE_INPUT, 25), 55312);
    }

    #[test]
    fn part1_alternative_example() {
        assert_eq!(part1_alternative(EXAMPLE_INPUT), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 55312);
    }

    #[test]
    fn num_of_digits() {
        assert_eq!((3 as f64).log10().floor() as usize + 1, 1);
        assert_eq!((10 as f64).log10().floor() as usize + 1, 2);
        assert_eq!((100 as f64).log10().floor() as usize + 1, 3);
    }
}
