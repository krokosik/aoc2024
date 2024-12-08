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

#[derive(Clone)]
struct Guard {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Guard {
    fn step(&mut self) {
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

#[derive(PartialEq, Clone)]
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

impl LabMap {
    fn new() -> Self {
        LabMap {
            positions: vec![],
            guard: Guard {
                x: 0,
                y: 0,
                dir: Direction::Right,
            },
            width: 0,
            height: 0,
        }
    }

    fn place_guard(&mut self, x: usize, y: usize, dir: Direction) {
        self.guard.x = x;
        self.guard.y = y;
        self.guard.dir = dir;
    }

    fn move_guard(&mut self) {
        let current_x = self.guard.x;
        let current_y = self.guard.y;

        self.guard.step();

        if self.guard_on_obstacle() {
            self.guard.y = current_y;
            self.guard.x = current_x;
            self.guard.dir = self.guard.dir.turn_right();
        } else {
            self.positions[current_y][current_x] = LabPosition::Patrolled;
        }
    }

    fn guard_on_obstacle(&self) -> bool {
        self.positions
            .get(self.guard.y)
            .and_then(|row| row.get(self.guard.x))
            == Some(&LabPosition::Obstacle)
    }

    fn guard_out_of_bounds(&self) -> bool {
        self.guard.x >= self.width || self.guard.y >= self.height
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> LabMap {
    input.lines().collect()
}

#[aoc(day6, part1)]
fn part1(input: &LabMap) -> usize {
    let mut lab_map = input.clone();

    while !lab_map.guard_out_of_bounds() {
        lab_map.move_guard();
    }

    lab_map
        .positions
        .iter()
        .flatten()
        .filter(|pos| pos == &&LabPosition::Patrolled)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator("....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...")), 41);
    }
}
