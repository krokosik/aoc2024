use std::ops::{Add, AddAssign, Mul, Neg};

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
