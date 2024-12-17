use std::collections::{BinaryHeap, HashMap};
use std::ops::IndexMut;
use std::{cmp::Ordering, ops::Index};

use crate::utils::{Direction, Pos};

#[derive(PartialEq, Clone, Copy)]
enum Field {
    Wall,
    Clear,
    Start,
    End,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '#' => Field::Wall,
            '.' => Field::Clear,
            'S' => Field::Start,
            'E' => Field::End,
            _ => panic!("Invalid field"),
        }
    }
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Vec<Vec<Field>> {
    input
        .lines()
        .map(|line| line.chars().map(Field::from).collect())
        .collect()
}

fn find_field(labirynth: &Vec<Vec<Field>>, field: Field) -> Pos {
    for (y, row) in labirynth.iter().enumerate() {
        for (x, f) in row.iter().enumerate() {
            if *f == field {
                return Pos {
                    x: x as i64,
                    y: y as i64,
                };
            }
        }
    }
    panic!("Field not found");
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    pos: Pos,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Index<Pos> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self[pos.y as usize][pos.x as usize]
    }
}

impl<T> IndexMut<Pos> for Vec<Vec<T>> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self[pos.y as usize][pos.x as usize]
    }
}

#[aoc(day16, part1)]
fn part1(labirynth: &Vec<Vec<Field>>) -> u64 {
    let start_pos = find_field(labirynth, Field::Start);
    let end_pos = find_field(labirynth, Field::End);
    let dir = Direction::Right;

    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::<(Pos, Direction), u64>::new();

    dist.insert((start_pos, dir), 0);

    heap.push(State {
        cost: 0,
        pos: start_pos,
        dir,
    });

    while let Some(State { cost, pos, dir }) = heap.pop() {
        if pos == end_pos {
            return cost;
        }

        if cost > dist.get(&(pos, dir)).copied().unwrap_or(u64::MAX) {
            continue;
        }

        if labirynth[pos + &dir] != Field::Wall {
            let next = State {
                cost: cost + 1,
                pos: pos + &dir,
                dir,
            };

            if next.cost < dist.get(&(next.pos, next.dir)).copied().unwrap_or(u64::MAX) {
                dist.insert((next.pos, next.dir), next.cost);
                heap.push(next);
            }
        }

        for &new_dir in [dir.turn_left(), dir.turn_right()].iter() {
            let next = State {
                cost: cost + 1000,
                pos,
                dir: new_dir,
            };

            if next.cost < dist.get(&(next.pos, next.dir)).copied().unwrap_or(u64::MAX) {
                dist.insert((next.pos, next.dir), next.cost);
                heap.push(next);
            }
        }
    }
    panic!("No path found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 7036);
    }
}