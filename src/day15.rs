use std::{fmt, ops::{Add, AddAssign, Index, IndexMut}};

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

#[derive(PartialEq, Clone, Copy)]
enum Field {
    Wall,
    Box,
    Clear,
    Robot,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '#' => Field::Wall,
            'O' => Field::Box,
            '.' => Field::Clear,
            '@' => Field::Robot,
            _ => panic!("Invalid field"),
        }
    }
}

impl<'a> From<&'a Field> for char {
    fn from(value: &'a Field) -> Self {
        match value {
            Field::Box => 'O',
            Field::Clear => '.',
            Field::Robot => '@',
            Field::Wall => '#',
        }
    }
}

#[derive(Clone)]
struct Warehouse {
    fields: Vec<Vec<Field>>,
    robot_pos: Pos,
    width: usize,
    height: usize,
}

impl Warehouse {
    fn new() -> Self {
        Warehouse {
            fields: Vec::new(),
            robot_pos: Pos { x: 0, y: 0 },
            width: 0,
            height: 0,
        }
    }

    fn place_robot(&mut self, x: isize, y: isize) {
        self.robot_pos = Pos { x, y };
    }

    fn move_robot(&mut self, dir: &Direction) {
        if let Some(field) = self.get_first_clear_field(dir) {
            let robot_pos = self.robot_pos.clone();
            self[&robot_pos] = Field::Clear;
            self[&(robot_pos + dir)] = Field::Robot;
            self.robot_pos += dir;
            if field != self.robot_pos {
                self[&field] = Field::Box;
            }
        }
    }

    fn get_first_clear_field(&self, dir: &Direction) -> Option<Pos> {
        let mut new_pos = self.robot_pos + dir;

        loop {
            match self[&new_pos] {
                Field::Clear => return Some(new_pos),
                Field::Wall => return None,
                _ => new_pos += dir,
            }
        }
    }

    fn get_box_coordinates(&self) -> u64 {
        self.fields
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, field)| match field {
                Field::Box => true,
                _ => false,
            })
            .map(|(i, _)| (i / self.width * 100 + i % self.width) as u64)
            .sum()
    }
}

impl<'a> FromIterator<&'a str> for Warehouse {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut warehouse = Warehouse::new();

        for (y, line) in iter.into_iter().enumerate() {
            warehouse.height += 1;
            warehouse.fields.push(Vec::with_capacity(warehouse.width));

            line.chars()
                .enumerate()
                .for_each(|(x, c)| match Field::from(c) {
                    Field::Robot => {
                        warehouse.place_robot(x as isize, y as isize);
                        warehouse.fields[y].push(Field::Robot);
                    }
                    pos => warehouse.fields[warehouse.height - 1].push(pos),
                });

            warehouse.width = warehouse.fields[y].len();
        }

        warehouse
    }
}

impl<'a> Index<&'a Pos> for Warehouse {
    type Output = Field;

    fn index(&self, pos: &'a Pos) -> &Self::Output {
        &self.fields[pos.y as usize][pos.x as usize]
    }
}

impl<'a> IndexMut<&'a Pos> for Warehouse {
    fn index_mut(&mut self, pos: &'a Pos) -> &mut Self::Output {
        &mut self.fields[pos.y as usize][pos.x as usize]
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.fields {
            let s: String = row.iter().map(|f| char::from(f)).collect();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[aoc(day15, part1)]
fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut wh: Warehouse = lines.by_ref().take_while(|l| !l.is_empty()).collect();

    lines.flat_map(|l| l.chars()).for_each(|c| {
        // println!("{}", wh);
        wh.move_robot(&c.into());
    });

    wh.get_box_coordinates()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn simple_example() {
        assert_eq!(part1(SIMPLE_INPUT), 2028);
    }
}