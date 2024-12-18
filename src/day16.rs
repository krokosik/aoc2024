use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

use itertools::Itertools;

use crate::utils::{Direction, Pos};

#[derive(PartialEq, Clone, Copy)]
enum Field {
    Wall,
    Clear,
    Start,
    End,
    Sit,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '#' => Field::Wall,
            '.' | 'O' => Field::Clear,
            'S' => Field::Start,
            'E' => Field::End,
            _ => panic!("Invalid field"),
        }
    }
}

impl From<Field> for char {
    fn from(value: Field) -> Self {
        match value {
            Field::Wall => '#',
            Field::Clear => '.',
            Field::Start => 'S',
            Field::End => 'E',
            Field::Sit => 'O',
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

impl State {
    fn neighbours(&self) -> [State; 3] {
        [
            State {
                cost: self.cost + 1,
                pos: self.pos + &self.dir,
                dir: self.dir,
            },
            State {
                cost: self.cost + 1000,
                pos: self.pos,
                dir: self.dir.turn_left(),
            },
            State {
                cost: self.cost + 1000,
                pos: self.pos,
                dir: self.dir.turn_right(),
            },
        ]
    }
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

fn get_dist(dist: &HashMap<(Pos, Direction), u64>, pos: Pos, dir: Direction) -> u64 {
    *dist.get(&(pos, dir)).unwrap_or(&u64::MAX)
}

fn dijkstra(
    labirynth: &Vec<Vec<Field>>,
    start_pos: Pos,
    start_dir: Direction,
    end_pos: Pos,
    dist: &mut HashMap<(Pos, Direction), u64>,
) {
    let mut heap = BinaryHeap::new();

    dist.insert((start_pos, start_dir), 0);

    heap.push(State {
        cost: 0,
        pos: start_pos,
        dir: start_dir,
    });

    let mut end_edges_count = (labirynth[end_pos - Direction::Right] == Field::Clear) as u64
        + (labirynth[end_pos - Direction::Up] == Field::Clear) as u64;

    while let Some(State { cost, pos, dir }) = heap.pop() {
        if pos == end_pos {
            end_edges_count -= 1;
            if end_edges_count == 0 {
                break;
            }
        }

        if cost > get_dist(dist, pos, dir) {
            continue;
        }

        for &next in (State { cost, pos, dir }).neighbours().iter() {
            if labirynth[next.pos] != Field::Wall && next.cost <= get_dist(dist, next.pos, next.dir)
            {
                dist.insert((next.pos, next.dir), next.cost);
                heap.push(next);
            }
        }
    }
}

#[aoc(day16, part1)]
fn part1(labirynth: &Vec<Vec<Field>>) -> u64 {
    let start_pos = find_field(labirynth, Field::Start);
    let end_pos = find_field(labirynth, Field::End);
    let dir = Direction::Right;

    let mut dist = HashMap::new();

    dijkstra(labirynth, start_pos, dir, end_pos, &mut dist);

    [Direction::Up, Direction::Right]
        .iter()
        .map(|&dir| dist.get(&(end_pos, dir)).unwrap_or(&u64::MAX).to_owned())
        .min()
        .unwrap()
}

#[aoc(day16, part2)]
fn part2(labirynth: &Vec<Vec<Field>>) -> u64 {
    let start_pos = find_field(labirynth, Field::Start);
    let end_pos = find_field(labirynth, Field::End);
    let dir = Direction::Right;

    let mut dist = HashMap::new();

    dijkstra(labirynth, start_pos, dir, end_pos, &mut dist);

    let min_dir = [Direction::Up, Direction::Right]
        .iter()
        .min_by_key(|&&dir| dist.get(&(end_pos, dir)).unwrap_or(&u64::MAX))
        .unwrap();

    let mut stack = vec![(end_pos, *min_dir)];
    let mut labirynth = labirynth.clone();

    labirynth[end_pos] = Field::Sit;

    while let Some((pos, dir)) = stack.pop() {
        if pos == start_pos {
            continue;
        }

        let prevs = [
            (pos - dir, dir),
            (pos, dir.turn_left()),
            (pos, dir.turn_right()),
        ]
        .iter()
        .map(|&(prev_pos, prev_dir)| (prev_pos, prev_dir, get_dist(&dist, prev_pos, prev_dir)))
        .filter(|&(_, _, d)| d != u64::MAX)
        .map(|(prev_pos, prev_dir, d)| {
            (
                prev_pos,
                prev_dir,
                d + if prev_pos != pos { 1 } else { 1000 },
            )
        })
        .collect_vec();

        let min_dist = prevs.iter().map(|prev| prev.2).min().unwrap();

        for &(prev_pos, prev_dir, _) in prevs.iter().filter(|&(_, _, d)| *d == min_dist) {
            labirynth[prev_pos] = Field::Sit;
            stack.push((prev_pos, prev_dir));
        }
    }

    for row in &labirynth {
        for &field in row {
            print!("{}", char::from(field));
        }
        println!();
    }

    labirynth
        .iter()
        .flatten()
        .filter(|&&f| f == Field::Sit)
        .count() as u64
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

    const SECOND_EXAMPLE_INPUT: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 7036);
        assert_eq!(part1(&input_generator(SECOND_EXAMPLE_INPUT)), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 45);
        assert_eq!(part2(&input_generator(SECOND_EXAMPLE_INPUT)), 64);
    }

    #[test]
    fn small_part2_test() {
        assert_eq!(
            part2(&input_generator(
                "#####
###E#
#...#
#.#.#
#...#
#S###
#####"
            )),
            10
        );
    }
}
