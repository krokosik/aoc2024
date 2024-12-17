use std::{
    collections::VecDeque,
    fmt,
    ops::{Index, IndexMut},
};

use crate::utils::{Direction, Pos};

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

    fn place_robot(&mut self, x: i64, y: i64) {
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
                        warehouse.place_robot(x as i64, y as i64);
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ScaledField {
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
    Clear,
}

impl<'a> From<&'a ScaledField> for char {
    fn from(value: &'a ScaledField) -> Self {
        match value {
            ScaledField::Wall => '#',
            ScaledField::BoxLeft => '[',
            ScaledField::BoxRight => ']',
            ScaledField::Robot => '@',
            ScaledField::Clear => '.',
        }
    }
}

struct ScaledWarehouse {
    fields: Vec<Vec<ScaledField>>,
    robot_pos: Pos,
    width: usize,
}

impl From<Warehouse> for ScaledWarehouse {
    fn from(wh: Warehouse) -> Self {
        let mut fields = Vec::with_capacity(wh.height);
        let mut robot_pos = wh.robot_pos;

        for (y, row) in wh.fields.iter().enumerate() {
            fields.push(Vec::with_capacity(wh.width * 2));

            for (x, field) in row.iter().enumerate() {
                fields[y].push(match field {
                    Field::Wall => ScaledField::Wall,
                    Field::Box => ScaledField::BoxLeft,
                    Field::Clear => ScaledField::Clear,
                    Field::Robot => {
                        robot_pos = Pos {
                            x: x as i64 * 2,
                            y: y as i64,
                        };
                        ScaledField::Robot
                    }
                });
                fields[y].push(match field {
                    Field::Wall => ScaledField::Wall,
                    Field::Box => ScaledField::BoxRight,
                    Field::Clear | Field::Robot => ScaledField::Clear,
                });
            }
        }

        ScaledWarehouse {
            fields,
            robot_pos,
            width: wh.width * 2,
        }
    }
}

impl<'a> Index<&'a Pos> for ScaledWarehouse {
    type Output = ScaledField;

    fn index(&self, pos: &'a Pos) -> &Self::Output {
        &self.fields[pos.y as usize][pos.x as usize]
    }
}

impl<'a> IndexMut<&'a Pos> for ScaledWarehouse {
    fn index_mut(&mut self, pos: &'a Pos) -> &mut Self::Output {
        &mut self.fields[pos.y as usize][pos.x as usize]
    }
}

impl fmt::Display for ScaledWarehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.fields {
            let s: String = row.iter().map(char::from).collect();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl ScaledWarehouse {
    fn move_robot(&mut self, dir: &Direction) {
        if self.move_boxes(dir) {
            let robot_pos = self.robot_pos.clone();
            self[&robot_pos] = ScaledField::Clear;
            self.robot_pos += dir;
            self[&(robot_pos + dir)] = ScaledField::Robot;
        }
    }

    fn move_boxes(&mut self, dir: &Direction) -> bool {
        let mut new_pos = self.robot_pos + dir;
        let inv_dir = &(-(*dir));

        if self[&new_pos] == ScaledField::Clear {
            return true;
        }

        match dir {
            Direction::Up | Direction::Down => {
                let mut stack = VecDeque::new();
                stack.push_back(new_pos);
                let mut visited = vec![new_pos];

                while let Some(pos) = stack.pop_front() {
                    match self[&pos] {
                        ScaledField::Wall => return false,
                        ScaledField::BoxLeft => {
                            for new_dir in [&Direction::Right, dir] {
                                let new_pos = pos + new_dir;
                                if !visited.contains(&new_pos) {
                                    stack.push_back(new_pos);
                                    visited.push(new_pos);
                                }
                            }
                            let new_pos = pos + dir + &Direction::Right;
                            if !visited.contains(&new_pos) {
                                stack.push_back(new_pos);
                                visited.push(new_pos);
                            }
                        }
                        ScaledField::BoxRight => {
                            for new_dir in [&Direction::Left, dir] {
                                let new_pos = pos + new_dir;
                                if !visited.contains(&new_pos) {
                                    stack.push_back(new_pos);
                                    visited.push(new_pos);
                                }
                            }
                            let new_pos = pos + dir + &Direction::Left;
                            if !visited.contains(&new_pos) {
                                stack.push_back(new_pos);
                                visited.push(new_pos);
                            }
                        }
                        _ => {}
                    }
                }

                while visited.len() > 2 {
                    let pos = visited.pop().unwrap();
                    if !visited.contains(&(pos + inv_dir)) {
                        continue;
                    }
                    let field = self[&pos];
                    self[&pos] = self[&(pos + inv_dir)];
                    self[&(pos + inv_dir)] = field;
                }

                while let Some(first_box_pos) = visited.pop() {
                    self[&first_box_pos] = ScaledField::Clear;
                }

                true
            }
            Direction::Left | Direction::Right => {
                loop {
                    match self[&new_pos] {
                        ScaledField::Clear => break,
                        ScaledField::Wall => return false,
                        _ => new_pos += dir,
                    }
                }
                while new_pos != self.robot_pos + dir {
                    let field = self[&new_pos];
                    self[&new_pos] = self[&(new_pos + inv_dir)];
                    self[&(new_pos + inv_dir)] = field;
                    new_pos += inv_dir;
                }
                true
            }
        }
    }

    fn get_box_coordinates(&self) -> u64 {
        self.fields
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, field)| match field {
                ScaledField::BoxLeft => true,
                _ => false,
            })
            .map(|(i, _)| (i / self.width * 100 + i % self.width) as u64)
            .sum()
    }

    #[allow(dead_code)]
    fn detect_broken_boxes(&self) -> bool {
        self.fields
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, field)| match field {
                ScaledField::BoxLeft => true,
                _ => false,
            })
            .any(|(i, _)| {
                let x = i % self.width;
                let y = i / self.width;
                let right = self.fields[y][x + 1];

                match right {
                    ScaledField::BoxRight => false,
                    _ => true,
                }
            })
    }
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let wh: Warehouse = lines.by_ref().take_while(|l| !l.is_empty()).collect();
    let mut swh: ScaledWarehouse = ScaledWarehouse::from(wh);

    lines.flat_map(|l| l.chars()).for_each(|c| {
        swh.move_robot(&c.into());
    });

    swh.get_box_coordinates()
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

    #[test]
    fn partial_wall_block() {
        assert_eq!(
            part2(
                "#######
#.....#
#.O.O@#
#..O..#
#..O..#
#.....#
#######

<v<<>vv<^^"
            ),
            822
        )
    }

    #[test]
    fn large_example2() {
        assert_eq!(
            part2(
                "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            9021
        );
    }
}
