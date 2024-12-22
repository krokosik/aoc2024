use std::{cmp::max, collections::VecDeque, iter::once};

use itertools::{repeat_n, Itertools};

use crate::utils::{Direction, Pos};

struct Keypad {
    keys: Vec<Vec<Option<char>>>,
    pos: Pos,
}

impl Keypad {
    fn numeric() -> Self {
        Self {
            keys: vec![
                vec![None; 5],
                vec![None, Some('7'), Some('8'), Some('9'), None],
                vec![None, Some('4'), Some('5'), Some('6'), None],
                vec![None, Some('1'), Some('2'), Some('3'), None],
                vec![None, None, Some('0'), Some('A'), None],
                vec![None; 5],
            ],
            pos: Pos { x: 3, y: 4 },
        }
    }

    fn directional() -> Self {
        Self {
            keys: vec![
                vec![None; 5],
                vec![None, None, Some('^'), Some('A'), None],
                vec![None, Some('<'), Some('v'), Some('>'), None],
                vec![None; 5],
            ],
            pos: Pos { x: 3, y: 1 },
        }
    }

    fn sequences_to_simplified(&mut self, target: char) -> Vec<char> {
        let start_pos = self.pos;
        let target_pos = self
            .keys
            .iter()
            .flatten()
            .position(|&c| c == Some(target))
            .unwrap();
        let target_pos = Pos {
            x: target_pos as i64 % 5,
            y: target_pos as i64 / 5,
        };
        self.pos = target_pos;

        let dx = target_pos.x - start_pos.x;
        let dy = target_pos.y - start_pos.y;

        repeat_n(if dx > 0 { '>' } else { '<' }, dx.abs() as usize)
            .chain(repeat_n(if dy > 0 { 'v' } else { '^' }, dy.abs() as usize))
            .chain(once('A'))
            .collect()
    }

    fn sequences_to(&mut self, target: char) -> Vec<char> {
        let mut queue = VecDeque::new();
        let mut paths = vec![];

        queue.push_back((self.pos, vec![], vec![self.pos]));

        while let Some((current_pos, path, ref mut visited)) = queue.pop_front() {
            if self.keys[current_pos] == Some(target) {
                let mut path = path;
                path.push('A');
                self.pos = current_pos;
                paths.push(path);
                continue;
            }

            if !paths.is_empty() && path.len() + 2 > paths[0].len() {
                continue;
            }

            for dir in Direction::into_iter() {
                let next = current_pos + dir;
                if !visited.contains(&next) && self.keys[next].is_some() {
                    let mut new_path = path.clone();
                    new_path.push(dir.into());
                    visited.push(next);
                    queue.push_back((next, new_path, visited.clone()));
                }
            }
        }

        let min_deduped = paths
            .iter()
            .map(|path| path.iter().dedup().dedup().count())
            .min()
            .unwrap();
        paths
            .into_iter()
            .filter(|path| path.iter().dedup().dedup().count() == min_deduped)
            .next()
            .unwrap()
    }
}

fn get_shortest_sequence(code: &Vec<char>, n_dir_keypads: usize) -> Vec<char> {
    let mut keypad_numeric = Keypad::numeric();

    let mut first_sequence_set = code
        .iter()
        .flat_map(|&c| keypad_numeric.sequences_to(c))
        .collect_vec();

    for _ in 0..n_dir_keypads {
        let mut keypad_directional = Keypad::directional();
        first_sequence_set = first_sequence_set
            .iter()
            .flat_map(|&c| keypad_directional.sequences_to_simplified(c))
            .collect_vec();
    }

    first_sequence_set
}

#[aoc(day21, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line[0..3].parse::<u64>().unwrap()
                * get_shortest_sequence(&line.chars().collect(), 2).len() as u64
        })
        .sum()
}

#[aoc(day21, part2)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| get_shortest_sequence(&line.chars().collect(), 25).len() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("029A"), 68 * 29);
        assert_eq!(part1("980A"), 60 * 980);
        assert_eq!(part1("179A"), 68 * 179);
        assert_eq!(part1("456A"), 64 * 456);
        assert_eq!(part1("379A"), 64 * 379);
    }
}
