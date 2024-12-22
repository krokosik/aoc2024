use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

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
            pos: Pos { x: 1, y: 2 },
        }
    }

    fn sequences_to(&mut self, target: char) -> Vec<Vec<char>> {
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

        paths
    }
}

#[aoc_generator(day21)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_shortest_sequence(code: &Vec<char>, n_dir_keypads: usize) -> Vec<char> {
    let mut keypad_numeric = Keypad::numeric();

    let first_sequence_set = code
        .iter()
        .flat_map(|&c| keypad_numeric.sequences_to(c))
        .collect_vec();

    println!("{:?}", first_sequence_set);

    first_sequence_set[0].clone()
}

#[aoc(day21, part1)]
fn part1(codes: &Vec<Vec<char>>) -> u64 {
    println!("{:?}", get_shortest_sequence(&codes[0], 1));
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator("029A")), 1);
    }
}
