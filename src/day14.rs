use std::fs::File;
use std::io::{self, Write};

use itertools::Itertools;

use crate::utils::Pos;

#[derive(Clone)]
struct Guard {
    p: Pos,
    v: Pos,
}

impl Guard {
    fn steps_wrapped(&mut self, steps: i64) {
        let s_width = W as i64;
        let s_height = H as i64;
        self.p += self.v * steps;
        self.p.x = ((self.p.x % s_width) + s_width) % s_width;
        self.p.y = ((self.p.y % s_height) + s_height) % s_height;
    }
}

impl From<&str> for Guard {
    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let p_coords: Vec<i64> = parts[0][2..]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let v_coords: Vec<i64> = parts[1][2..]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Guard {
            p: Pos {
                x: p_coords[0],
                y: p_coords[1],
            },
            v: Pos {
                x: v_coords[0],
                y: v_coords[1],
            },
        }
    }
}

fn get_quadrant_counts(guards: &[Guard]) -> u64 {
    get_quadrant_iters(guards)
        .iter_mut()
        .map(|it| it.as_mut().count() as u64)
        .product()
}

fn get_quadrant_iters<'a>(guards: &'a [Guard]) -> [Box<dyn Iterator<Item = &'a Pos> + 'a>; 4] {
    let hw = W as i64 / 2;
    let hh = H as i64 / 2;
    [
        Box::new(
            guards
                .iter()
                .map(|g| &g.p)
                .filter(move |p| p.x < hw && p.y < hh),
        ),
        Box::new(
            guards
                .iter()
                .map(|g| &g.p)
                .filter(move |p| p.x > hw && p.y < hh),
        ),
        Box::new(
            guards
                .iter()
                .map(|g| &g.p)
                .filter(move |p| p.x < hw && p.y > hh),
        ),
        Box::new(
            guards
                .iter()
                .map(|g| &g.p)
                .filter(move |p| p.x > hw && p.y > hh),
        ),
    ]
}

const W: usize = 101;
const H: usize = 103;

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let mut guards = input.lines().map(Guard::from).collect_vec();

    for g in guards.iter_mut() {
        g.steps_wrapped(100);
    }

    get_quadrant_counts(&guards)
}

fn save_robot_map(guards: &[Guard], id: usize) -> io::Result<()> {
    let mut map = Vec::with_capacity(H);

    for _ in 0..H {
        map.push(vec![0; W]);
    }

    for g in guards {
        map[g.p.y as usize][g.p.x as usize] += 1;
    }

    let mut file = File::create(format!("day14-step{}.txt", id))?;

    for row in map.iter() {
        for &field in row.iter() {
            if field == 0 {
                write!(file, ".")?;
            } else {
                write!(file, "{}", field)?;
            }
        }
        writeln!(file, "")?;
    }
    Ok(())
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut guards = input.lines().map(Guard::from).collect_vec();

    let mut min_score = 999999999999;
    let mut min_i: u64 = 0;
    for i in 0..W * H {
        for g in guards.iter_mut() {
            g.steps_wrapped(1);
        }
        let score: u64 = get_quadrant_counts(&guards);
        if score < min_score {
            min_score = score;
            min_i = 1 + i as u64;
            println!("Step: {}, Score: {}", min_i, min_score);
            save_robot_map(&guards, min_i as usize).unwrap();
        }
    }
    min_i
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_trivial_christmas_tree() {
        assert_eq!(part2("p=0,0 v=0,0"), 0)
    }

    #[test]
    fn test_small_christmas_tree() {
        assert_eq!(
            part2(
                "p=5,10 v=0,0
p=6,9 v=0,0
p=4,9 v=0,0
p=5,9 v=0,0"
            ),
            0
        )
    }

    #[test]
    fn test_larger_christmas_tree() {
        assert_eq!(
            part2(
                "p=4,10 v=1,0
p=6,9 v=0,0
p=4,9 v=0,0
p=5,9 v=0,0
p=6,8 v=0,0
p=4,8 v=0,0
p=5,8 v=0,0
p=3,8 v=0,0
p=7,8 v=0,0"
            ),
            1
        )
    }
}
