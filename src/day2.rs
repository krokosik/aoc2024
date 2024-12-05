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
        if report.is_empty() {
            sum + 1
        } else {
            if let Continue(_) = check_report_monotonous(report.iter(), with_dampener) {
                sum + 1
            } else if let Continue(_) = check_report_monotonous(report.iter().rev(), with_dampener)
            {
                sum + 1
            } else {
                sum
            }
        }
    })
}

fn check_report_monotonous<'a>(
    report: impl Iterator<Item = &'a u32>,
    with_dampener: bool,
) -> ControlFlow<(), (Option<&'a u32>, Option<&'a u32>)> {
    let mut dampener_available = with_dampener;

    report.collect::<Vec<&u32>>().iter().try_fold(
        (None, None),
        |(pprev, prev): (Option<&u32>, Option<&u32>), &level| {
            println!("{:?} {:?} {:?} {}", pprev, prev, level, dampener_available);
            match (pprev, prev, level, dampener_available) {
                (_, Some(prev), level, _) if condition(prev, level) => {
                    Continue((Some(prev), Some(level)))
                }
                (_, Some(prev), level, false) if !condition(prev, level) => Break(()),
                (Some(pprev), _, level, true) if condition(pprev, level) => {
                    dampener_available = false;
                    Continue((Some(pprev), Some(level)))
                }
                (Some(pprev), Some(prev), _, true) if condition(pprev, prev) => {
                    dampener_available = false;
                    Continue((Some(pprev), Some(prev)))
                }
                (None, prev, level, _) => Continue((prev, Some(level))),
                _ => Break(()),
            }
        },
    )
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
