use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_left(self) -> Self {
        -self.turn_right()
    }

    pub fn into_iter() -> core::array::IntoIter<Direction, 4> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .into_iter()
    }
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

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub fn manhattan_distance(&self, other: Pos) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u64
    }
}

impl From<(i64, i64)> for Pos {
    fn from((x, y): (i64, i64)) -> Self {
        Pos { x, y }
    }
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

impl Add<Direction> for Pos {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        self + &dir
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

impl AddAssign<Direction> for Pos {
    fn add_assign(&mut self, dir: Direction) {
        *self += &dir;
    }
}

impl Sub<Direction> for Pos {
    type Output = Self;

    fn sub(self, dir: Direction) -> Self {
        self + -dir
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, other: Pos) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Mul<i64> for Pos {
    type Output = Pos;

    fn mul(self, other: i64) -> Pos {
        Pos {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(std::cmp::Ordering::Equal) => self.x.partial_cmp(&other.x),
            res => res,
        }
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
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