use std::{
    cmp::min,
    ops::{Add, AddAssign, Mul},
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
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

impl Mul<usize> for Pos {
    type Output = Pos;

    fn mul(self, other: usize) -> Pos {
        Pos {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Machine {
    prize: Pos,
    button_a: Pos,
    button_b: Pos,
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
                button_a: positions.next().unwrap(),
                button_b: positions.next().unwrap(),
                prize: positions.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[Machine]) -> usize {
    input
        .iter()
        .flat_map(|machine| (0..=100).map(|n| find_cheapest_path(machine, n)))
        .sum()
}

fn find_cheapest_path(machine: &Machine, n_steps: usize) -> usize {
    (0..=2 * n_steps)
        .map(|n_a| (min(n_a, n_steps), min(2 * n_steps - n_a, n_steps)))
        .filter(|&(n_a, n_b)| machine.button_a * n_a + machine.button_b * n_b == machine.prize)
        .map(|(n_a, n_b)| n_a * 3 + n_b * 1)
        .min()
        .unwrap_or(0)
}

#[aoc(day13, part2)]
fn part2(_input: &[Machine]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_generator() {
        let input = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n";
        let expected = vec![Machine {
            button_a: Pos { x: 94, y: 34 },
            button_b: Pos { x: 22, y: 67 },
            prize: Pos { x: 8400, y: 5400 },
        }];
        assert_eq!(input_generator(input), expected);
    }

    #[test]
    fn part1_example() {
        let input = input_generator(
            "Button A: X+94, Y+34
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
Prize: X=18641, Y=10279",
        );
        assert_eq!(part1(&input), 480);
    }
}
