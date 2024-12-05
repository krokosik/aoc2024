use std::ops::ControlFlow::{self, Break, Continue};

type Input = Vec<Vec<u32>>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Input) -> u32 {
    safety_check(input, false)
}

#[aoc(day2, part2)]
pub fn part2(input: &Input) -> u32 {
    safety_check(input, true)
}

fn safety_check(input: &Input, with_dampener: bool) -> u32 {
    input.iter().fold(0, |sum, report| {
        if check_report_monotonous(report, false, with_dampener)
            || check_report_monotonous(report, true, with_dampener)
        {
            sum + 1
        } else {
            sum
        }
    })
}

fn check_report_monotonous(report: &Vec<u32>, ascending: bool, with_dampener: bool) -> bool {
    let report: Vec<_> = if ascending {
        report.clone()
    } else {
        report.iter().cloned().rev().collect()
    };

    report.iter().try_fold(None, check_neigbours).is_continue()
        || (with_dampener
            && (0..report.len()).any(|i| {
                report[0..i]
                    .iter()
                    .chain(&report[i + 1..])
                    .try_fold(None, check_neigbours)
                    .is_continue()
            }))
}

fn check_neigbours<'a>(prev: Option<&'a u32>, level: &'a u32) -> ControlFlow<(), Option<&'a u32>> {
    match (prev, level) {
        (Some(prev), level) if condition(prev, level) => Continue(Some(level)),
        (Some(prev), level) if !condition(prev, level) => Break(()),
        (None, level) => Continue(Some(level)),
        _ => Break(()),
    }
}

fn condition(prev: &u32, x: &u32) -> bool {
    prev < x && prev + 3 >= *x
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn input_generator_example() {
        assert_eq!(
            input_generator(EXAMPLE_INPUT),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 4);
    }

    #[test]
    fn first_level_dampable() {
        assert_eq!(safety_check(&vec![vec![1, 6, 9, 10, 11]], true), 1);
    }

    #[test]
    fn second_level_dampable() {
        assert_eq!(safety_check(&vec![vec![6, 4, 9, 10, 11]], true), 1);
    }

    #[test]
    fn third_level_dampable() {
        assert_eq!(safety_check(&vec![vec![4, 6, 4, 9, 10]], true), 1);
    }

    #[test]
    fn last_level_dampable() {
        assert_eq!(safety_check(&vec![vec![1, 2, 3, 10]], true), 1);
    }

    #[test]
    fn undampable() {
        assert_eq!(safety_check(&vec![vec![1, 2, 7, 8, 9]], true), 0);
    }

    #[test]
    fn undampable2() {
        assert_eq!(safety_check(&vec![vec![1, 4, 8, 10]], true), 0);
    }
}
