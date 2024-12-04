use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let first = iter.next().unwrap().parse::<u32>().unwrap();
            let second = iter.next().unwrap().parse::<u32>().unwrap();
            (first, second)
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (mut a, mut b) = input.clone();
    a.sort();
    b.sort();
    std::iter::zip(a.iter(), b.iter()).fold(0, |acc, (a, b)| acc + u32::abs_diff(*a, *b))
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let map_a = get_number_counts(&input.0);
    let map_b = get_number_counts(&input.1);

    map_a.iter().fold(0, |acc, (k, v)| {
        acc + map_b.get(k).map_or(0, |v2| k * v * v2)
    })
}

fn get_number_counts(input: &Vec<u32>) -> HashMap<u32, u32> {
    input.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "1721 979";
        assert_eq!(input_generator(input), (vec![1721], vec![979]));
    }

    const EXAMPLE_INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 31);
    }
}
