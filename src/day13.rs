use std::{
    cmp::min,
    ops::{Add, AddAssign, Mul},
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, other: Pos) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Mul<i64> for Pos {
    type Output = Pos;

    fn mul(self, other: i64) -> Pos {
        Pos {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Machine {
    prize: Pos,
    a: Pos,
    b: Pos,
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<Machine> {
    let re = Regex::new(r"X[+-=](?<X>\d+), Y[+-=](?<Y>\d+)").unwrap();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let mut positions = chunk.take(3).map(|line| {
                let caps = re.captures(line).unwrap();
                Pos {
                    x: caps["X"].parse().unwrap(),
                    y: caps["Y"].parse().unwrap(),
                }
            });
            Machine {
                a: positions.next().unwrap(),
                b: positions.next().unwrap(),
                prize: positions.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day13, part1, brute_force)]
fn part1(input: &[Machine]) -> i64 {
    input
        .iter()
        .flat_map(|machine| (0..=100).map(|n| find_cheapest_path(machine, n)))
        .sum()
}

fn find_cheapest_path(machine: &Machine, n_steps: i64) -> i64 {
    (0..=2 * n_steps)
        .map(|n_a| {
            (
                min(n_a, n_steps) as i64,
                min(2 * n_steps - n_a, n_steps) as i64,
            )
        })
        .filter(|&(n_a, n_b)| machine.a * n_a + machine.b * n_b == machine.prize)
        .map(|(n_a, n_b)| n_a * 3 + n_b * 1)
        .min()
        .unwrap_or(0)
}

#[aoc(day13, part1, exact)]
fn part1_exact(input: &[Machine]) -> i64 {
    input
        .iter()
        .map(|machine| calculate_tokens_with_offset(machine, 0))
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Machine]) -> i64 {
    input
        .iter()
        .map(|machine| calculate_tokens_with_offset(machine, 10000000000000))
        .sum()
}

fn calculate_tokens_with_offset(m: &Machine, offset: i64) -> i64 {
    let n_a_counter = ((m.prize.x + offset) * m.b.y) - ((m.prize.y + offset) * m.b.x);

    let n_a_denominator = (m.a.x * m.b.y) - (m.a.y * m.b.x);

    if n_a_denominator == 0
        || n_a_counter / n_a_denominator < 0
        || n_a_counter % n_a_denominator != 0
    {
        return 0;
    }

    let n_a = n_a_counter / n_a_denominator;

    let n_b_counter = (m.prize.x + offset) - (n_a * m.a.x);
    let n_b_denominator = m.b.x;
    if n_b_denominator == 0
        || n_b_counter / n_b_denominator < 0
        || n_b_counter % n_b_denominator != 0
    {
        return 0;
    }

    let n_b = n_b_counter / n_b_denominator;

    n_a * 3 + n_b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_generator() {
        let input = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n";
        let expected = vec![Machine {
            a: Pos { x: 94, y: 34 },
            b: Pos { x: 22, y: 67 },
            prize: Pos { x: 8400, y: 5400 },
        }];
        assert_eq!(input_generator(input), expected);
    }

    const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part1_exact(&input_generator(EXAMPLE_INPUT)), 480);
    }
}
