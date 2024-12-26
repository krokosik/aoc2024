use std::{
    collections::HashMap,
    iter::once,
};

use itertools::{repeat_n, Itertools};

use crate::utils::Pos;

struct Keypad {
    keys: Vec<Vec<Option<char>>>,
    pos: Pos,
}

type FrequencyTable = HashMap<(char, char), u64>;

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

    fn get_coordinates(&self, c: char) -> Pos {
        let pos = self
            .keys
            .iter()
            .flatten()
            .position(|&key| key == Some(c))
            .unwrap();
        Pos {
            x: pos as i64 % self.keys[0].len() as i64,
            y: pos as i64 / self.keys[0].len() as i64,
        }
    }

    fn sequences_to(&mut self, target: char) -> Vec<char> {
        let start_pos = self.pos;
        let target_pos = self.get_coordinates(target);
        self.pos = target_pos;

        let dx = target_pos.x - start_pos.x;
        let dy = target_pos.y - start_pos.y;

        let h = repeat_n(if dx > 0 { '>' } else { '<' }, dx.abs() as usize);
        let v = repeat_n(if dy > 0 { 'v' } else { '^' }, dy.abs() as usize);
        let a = once('A');
        (if dx > 0 && !self.keys[target_pos.y as usize][start_pos.x as usize].is_none() {
            v.chain(h)
        } else if !self.keys[start_pos.y as usize][target_pos.x as usize].is_none() {
            h.chain(v)
        } else {
            v.chain(h)
        })
        .chain(a)
        .collect()
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
            .flat_map(|&c| keypad_directional.sequences_to(c))
            .collect_vec();
    }

    first_sequence_set
}

fn update_frequency_table(table: &mut FrequencyTable) -> FrequencyTable {
    let mut new_table = HashMap::new();

    for &(start, target) in table.keys() {
        let moves = get_optimal_directional_path(start, target);
        let mut prev = 'A';
        for &m in &moves {
            *new_table.entry((prev, m)).or_insert(0) += table[&(start, target)];
            prev = m;
        }
    }

    new_table
}

fn get_optimal_directional_path(start: char, target: char) -> Vec<char> {
    (match (start, target) {
        (start, target) if start == target => "",
        ('A', '^') | ('>', 'v') | ('v', '<') => "<",
        ('A', '>') | ('^', 'v') => "v",
        ('A', 'v') => "<v",
        ('A', '<') => "v<<",
        ('^', 'A') | ('<', 'v') | ('v', '>') => ">",
        ('>', 'A') => "^",
        ('<', 'A') => ">>^",
        ('v', 'A') => "^>",
        ('^', '>') => "v>",
        ('<', '^') => ">^",
        ('>', '^') => "<^",
        ('<', '>') => ">>",
        ('>', '<') => "<<",
        ('^', '<') => "v<",
        _ => unreachable!(),
    })
    .chars()
    .chain(once('A'))
    .collect()
}

fn use_freq_tables(input: &str, n: usize) -> u64 {
    let mut keypad_numeric = Keypad::numeric();
    let sequences = input
        .lines()
        .flat_map(|line| line.chars())
        .flat_map(|c| keypad_numeric.sequences_to(c))
        .collect_vec();

    let mut freq_table = HashMap::new();

    freq_table.insert(('A', sequences[0]), 1);
    for pair in sequences.windows(2) {
        *freq_table.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    let freq_table = (0..n).fold(freq_table, |mut table, _| {
        update_frequency_table(&mut table)
    });
    freq_table.values().sum()
}

#[aoc(day21, part1, loop)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line[0..3].parse::<u64>().unwrap()
                * get_shortest_sequence(&line.chars().collect(), 2).len() as u64
        })
        .sum()
}

#[aoc(day21, part1, freq_tables)]
fn part12(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line[0..3].parse::<u64>().unwrap() * use_freq_tables(line, 2) as u64)
        .sum()
}

#[aoc(day21, part2)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line[0..3].parse::<u64>().unwrap() * use_freq_tables(line, 25) as u64)
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
