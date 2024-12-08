use std::{
    ops::{Add, AddAssign, Index, IndexMut},
    vec,
};

#[derive(Clone)]
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

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl<'a> Add<&'a Direction> for Pos {
    type Output = Self;

    fn add(self, dir: &'a Direction) -> Self {
        match dir {
            Direction::Up => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Pos {
                x: self.x + 1,
                y: self.y,
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

#[derive(Clone)]
struct Guard {
    pos: Pos,
    dir: Direction,
}

#[derive(PartialEq, Clone, Copy)]
enum LabPosition {
    Clear,
    Patrolled,
    Obstacle,
    Guard,
}

impl From<char> for LabPosition {
    fn from(c: char) -> Self {
        match c {
            '.' => LabPosition::Clear,
            'X' => LabPosition::Patrolled,
            '#' => LabPosition::Obstacle,
            '^' | 'v' | '<' | '>' => LabPosition::Guard,
            _ => panic!("Invalid lab position"),
        }
    }
}

#[derive(Clone)]
struct LabMap {
    positions: Vec<Vec<LabPosition>>,
    guard: Guard,
    width: usize,
    height: usize,
}

impl<'a> FromIterator<&'a str> for LabMap {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut lab_map = LabMap::new();

        for (y, line) in iter.into_iter().enumerate() {
            lab_map.height += 1;
            lab_map.positions.push(Vec::with_capacity(lab_map.width));

            line.chars()
                .enumerate()
                .for_each(|(x, c)| match LabPosition::from(c) {
                    LabPosition::Guard => {
                        lab_map.place_guard(x, y, Direction::from(c));
                        lab_map.positions[y].push(LabPosition::Guard);
                    }
                    pos => lab_map.positions[lab_map.height - 1].push(pos),
                });

            lab_map.width = lab_map.positions[y].len();
        }

        lab_map
    }
}

impl Index<Pos> for LabMap {
    type Output = LabPosition;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.positions[pos.y][pos.x]
    }
}

impl IndexMut<Pos> for LabMap {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.positions[pos.y][pos.x]
    }
}

impl LabMap {
    fn new() -> Self {
        LabMap {
            positions: vec![],
            guard: Guard {
                pos: Pos { x: 0, y: 0 },
                dir: Direction::Right,
            },
            width: 0,
            height: 0,
        }
    }

    fn place_guard(&mut self, x: usize, y: usize, dir: Direction) {
        self.guard.pos.x = x;
        self.guard.pos.y = y;
        self.guard.dir = dir;
    }

    fn move_guard(&mut self) {
        let current_pos = self.guard.pos;

        self.guard.pos += &self.guard.dir;

        if self.guard_on_obstacle() {
            self.guard.pos = current_pos;
            self.guard.dir = self.guard.dir.turn_right();
        } else {
            self[current_pos] = LabPosition::Patrolled;
        }
    }

    fn guard_on_obstacle(&self) -> bool {
        self.positions
            .get(self.guard.pos.y)
            .and_then(|row| row.get(self.guard.pos.x))
            == Some(&LabPosition::Obstacle)
    }

    fn out_of_bounds(&self, pos: &Pos) -> bool {
        pos.x >= self.width || pos.y >= self.height
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> LabMap {
    input.lines().collect()
}

#[aoc(day6, part1)]
fn part1(input: &LabMap) -> usize {
    let mut lab_map = input.clone();

    while !lab_map.out_of_bounds(&lab_map.guard.pos) {
        lab_map.move_guard();
    }

    lab_map
        .positions
        .iter()
        .flatten()
        .filter(|pos| pos == &&LabPosition::Patrolled)
        .count()
}

#[aoc(day6, part2)]
fn part2(input: &LabMap) -> usize {
    0
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
    fn test_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 6);
    }
}
