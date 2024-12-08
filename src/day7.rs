use std::ops::{Add, Mul};

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

fn is_valid<T>(rhs: T, lhs: T, args: &[T]) -> bool
where
    T: PartialOrd + Copy,
    T: Add<Output = T> + Mul<Output = T>,
{
    if args.is_empty() && rhs == lhs {
        true
    } else if args.is_empty() || rhs > lhs {
        false
    } else {
        let remaining = &args[1..];
        is_valid(rhs + args[0], lhs, remaining) || is_valid(rhs * args[0], lhs, remaining)
    }
}

fn is_valid_with_concat(rhs: u64, lhs: u64, args: &[u64]) -> bool
// where
//     T: PartialOrd + Copy,
//     T: Add<Output = T> + Mul<Output = T>,
{
    if args.is_empty() && rhs == lhs {
        true
    } else if args.is_empty() || rhs > lhs {
        false
    } else {
        let remaining = &args[1..];
        let concatenated_rhs = format!("{}{}", rhs, args[0]).parse().unwrap();
        is_valid_with_concat(concatenated_rhs, lhs, remaining)
            || is_valid_with_concat(rhs + args[0], lhs, remaining)
            || is_valid_with_concat(rhs * args[0], lhs, remaining)
    }
}

#[aoc(day7, part1)]
fn part1(input: &Vec<Equation>) -> u64 {
    input
        .iter()
        .filter(|(lhs, args)| is_valid(0, *lhs, args))
        .map(|(lhs, _)| *lhs)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &Vec<Equation>) -> u64 {
    input
        .iter()
        .filter(|(lhs, args)| is_valid_with_concat(0, *lhs, args))
        .map(|(lhs, _)| *lhs)
        .sum()
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
