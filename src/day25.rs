use std::iter::zip;

use itertools::repeat_n;

type Key = Vec<usize>;
type Lock = Vec<usize>;

const HEIGHT: usize = 5;

#[aoc_generator(day25)]
fn input_generator(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let lines = input.lines();
    let mut tmp = vec![];
    let mut is_key = false;

    for line in lines {
        if tmp.is_empty() {
            is_key = line.chars().all(|c| c == '#');
            tmp.extend(repeat_n(if is_key { 1 } else { 0 }, line.len()));
        } else if line.is_empty() {
            for i in 0..tmp.len() {
                tmp[i] -= 1;
            }
            if is_key {
                keys.push(tmp.clone());
            } else {
                locks.push(tmp.clone());
            }
            tmp.clear();
        } else {
            for (i, c) in line.chars().enumerate() {
                tmp[i] += if c == '#' { 1 } else { 0 };
            }
        }
    }

    for i in 0..tmp.len() {
        tmp[i] -= 1;
    }
    if is_key {
        keys.push(tmp.clone());
    } else {
        locks.push(tmp.clone());
    }

    (keys, locks)
}

#[aoc(day25, part1)]
fn part1((keys, locks): &(Vec<Key>, Vec<Lock>)) -> usize {
    let mut res = 0;

    keys.iter().for_each(|key| {
        locks.iter().for_each(|lock| {
            res += zip(key, lock).all(|(k, l)| k + l <= HEIGHT) as usize;
        });
    });

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let input = input_generator(input);
        assert_eq!(part1(&input), 3);
    }
}
