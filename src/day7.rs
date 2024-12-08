use std::{fmt::{Debug, Display}, ops::{Add, Mul}, str::FromStr};

type Equation = (u64, Vec<u64>);

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let res = parts[0].parse().unwrap();
            let args = parts[1]
                .split_whitespace()
                .map(|arg| arg.parse().unwrap())
                .collect();
            (res, args)
        })
        .collect()
}

fn is_valid<T, F>(rhs: T, lhs: T, args: &[T], ops: &[F]) -> bool
where
    T: PartialOrd + Copy,
    T: Add<Output = T> + Mul<Output = T>,
    F: Fn(T, T) -> T,
{
    if args.is_empty() && rhs == lhs {
        true
    } else if args.is_empty() || rhs > lhs {
        false
    } else {
        let remaining = &args[1..];
        ops.iter()
            .any(|op| is_valid(op(rhs, args[0]), lhs, remaining, ops))
    }
}

#[aoc(day7, part1)]
fn part1(input: &Vec<Equation>) -> u64 {
    input
        .iter()
        .filter(|(lhs, args)| is_valid(0, *lhs, args, &vec![Add::add, Mul::mul]))
        .map(|(lhs, _)| *lhs)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &Vec<Equation>) -> u64 {
    input
        .iter()
        .filter(|(lhs, args)| is_valid(0, *lhs, args, &vec![Add::add, Mul::mul, concat]))
        .map(|(lhs, _)| *lhs)
        .sum()
}

fn concat<T>(left: T, right: T) -> T
where
    T: Display + FromStr,
    <T as FromStr>::Err: Debug
{
    format!("{}{}", left, right).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 3749);
    }

    #[test]
    fn concat_test() {
        assert_eq!(part2(&input_generator("156: 15 6")), 156);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 11387);
    }
}
