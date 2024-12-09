use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Index, IndexMut},
    vec,
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: isize,
    y: isize,
    dir: Direction,
}

impl Pos {
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn equals_without_dir(&self, other: &Pos) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<'a> Add<&'a Direction> for Pos {
    type Output = Self;

    fn add(self, dir: &'a Direction) -> Self {
        match dir {
            Direction::Up => Pos {
                x: self.x,
                y: self.y - 1,
                dir: self.dir,
            },
            Direction::Down => Pos {
                x: self.x,
                y: self.y + 1,
                dir: self.dir,
            },
            Direction::Left => Pos {
                x: self.x - 1,
                y: self.y,
                dir: self.dir,
            },
            Direction::Right => Pos {
                x: self.x + 1,
                y: self.y,
                dir: self.dir,
            },
        }
    }
}

impl<'a> AddAssign<&'a Direction> for Pos {
    fn add_assign(&mut self, dir: &'a Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum LabField {
    Clear,
    Patrolled,
    Obstacle,
    Guard,
}

impl From<char> for LabField {
    fn from(c: char) -> Self {
        match c {
            '.' => LabField::Clear,
            'X' => LabField::Patrolled,
            '#' => LabField::Obstacle,
            '^' | 'v' | '<' | '>' => LabField::Guard,
            _ => panic!("Invalid lab position"),
        }
    }
}

#[derive(Clone)]
struct LabMap {
    fields: Vec<Vec<LabField>>,
    guard_pos: Pos,
    width: usize,
    height: usize,
}

impl<'a> FromIterator<&'a str> for LabMap {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut lab_map = LabMap::new();

        for (y, line) in iter.into_iter().enumerate() {
            lab_map.height += 1;
            lab_map.fields.push(Vec::with_capacity(lab_map.width));

            line.chars()
                .enumerate()
                .for_each(|(x, c)| match LabField::from(c) {
                    LabField::Guard => {
                        lab_map.place_guard(x as isize, y as isize, Direction::from(c));
                        lab_map.fields[y].push(LabField::Guard);
                    }
                    pos => lab_map.fields[lab_map.height - 1].push(pos),
                });

            lab_map.width = lab_map.fields[y].len();
        }

        lab_map
    }
}

impl Index<Pos> for LabMap {
    type Output = LabField;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.fields[pos.y as usize][pos.x as usize]
    }
}

impl IndexMut<Pos> for LabMap {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.fields[pos.y as usize][pos.x as usize]
    }
}

impl LabMap {
    fn new() -> Self {
        LabMap {
            fields: vec![],
            guard_pos: Pos {
                x: 0,
                y: 0,
                dir: Direction::Right,
            },
            width: 0,
            height: 0,
        }
    }

    fn place_guard(&mut self, x: isize, y: isize, dir: Direction) {
        self.guard_pos = Pos { x, y, dir };
    }

    fn move_guard(&mut self) {
        let current_pos = self.guard_pos.clone();

        self.guard_pos += &current_pos.dir;

        if !self.out_of_bounds() && self[self.guard_pos] == LabField::Obstacle {
            self.guard_pos = current_pos;
            self.guard_pos.turn_right();
        } else {
            self[current_pos] = LabField::Patrolled;
        }
    }

    fn out_of_bounds(&self) -> bool {
        self.guard_pos.x >= self.width as isize
            || self.guard_pos.y >= self.height as isize
            || self.guard_pos.x < 0
            || self.guard_pos.y < 0
    }
}

impl Iterator for LabMap {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.out_of_bounds() {
            return None;
        }
        self.move_guard();
        Some(self.guard_pos)
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> LabMap {
    input.lines().collect()
}

#[aoc(day6, part1)]
fn part1(input: &LabMap) -> usize {
    let mut lab_map = input.clone();

    while let Some(_) = lab_map.next() {}

    lab_map
        .fields
        .iter()
        .flatten()
        .filter(|pos| pos == &&LabField::Patrolled)
        .count()
}

#[aoc(day6, part2)]
fn part2(input: &LabMap) -> usize {
    let lab_map = input.clone();

    let mut obstacle_placements = HashSet::<Pos>::new();
    obstacle_placements.insert(lab_map.guard_pos);
    let guard_route = lab_map.collect_vec();

    for i in 0..guard_route.len() - 2 {
        let mut lab_map = input.clone();
        let mut current_guard_route = guard_route[..=i].to_vec();
        if input[guard_route[i + 1]] == LabField::Obstacle
            || current_guard_route
                .iter()
                .any(|pos| pos.equals_without_dir(&guard_route[i + 1]))
        {
            continue;
        }
        lab_map.guard_pos = guard_route[i];
        lab_map[guard_route[i + 1]] = LabField::Obstacle;
        lab_map.move_guard();

        while !lab_map.out_of_bounds() && !current_guard_route.contains(&lab_map.guard_pos) {
            current_guard_route.push(lab_map.guard_pos);
            lab_map.move_guard();
        }

        if !lab_map.out_of_bounds() {
            obstacle_placements.insert(guard_route[i + 1]);
        }
    }

    obstacle_placements.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 41);
    }

    #[test]
    fn test_pos_eq() {
        assert_ne!(
            Pos {
                x: 1,
                y: 1,
                dir: Direction::Down
            },
            Pos {
                x: 1,
                y: 1,
                dir: Direction::Up
            }
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 6);
    }
}
