use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{Direction, Pos};

fn get_path(race_track: &Vec<Vec<char>>) -> Vec<Pos> {
    let start_idx = race_track.iter().flatten().position(|&c| c == 'S').unwrap();
    let end_idx = race_track.iter().flatten().position(|&c| c == 'E').unwrap();
    let w = race_track[0].len();

    let start_pos = Pos {
        x: (start_idx % w) as i64,
        y: (start_idx / w) as i64,
    };
    let end_pos = Pos {
        x: (end_idx % w) as i64,
        y: (end_idx / w) as i64,
    };

    let mut cur = start_pos;

    let mut path = vec![start_pos];

    while cur != end_pos {
        for next in Direction::into_iter().map(|dir| cur + dir) {
            if race_track[next] != '#' && (path.len() < 2 || path[path.len() - 2] != next) {
                cur = next;
                path.push(cur);
                break;
            }
        }
    }

    path
}

fn get_neighbours_in_manhattan_range(pos: Pos, range: i64) -> impl Iterator<Item = Pos> {
    (-range..=range)
        .cartesian_product(-range..=range)
        .filter(move |(i, j)| i.abs() + j.abs() <= range)
        .map(move |(i, j)| pos + Pos::from((i, j)))
}

fn get_possible_cheat_gains(race_track: &Vec<Vec<char>>, min_gain: usize, range: i64) -> usize {
    let path = get_path(&race_track);

    let costs = path
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i))
        .collect::<HashMap<_, _>>();

    path.iter()
        .copied()
        .enumerate()
        .flat_map(|(i, pos)| {
            let costs = &costs;
            get_neighbours_in_manhattan_range(pos, range)
                .filter_map(move |next| {
                    costs.get(&next).and_then(|&cost| {
                        cost.checked_sub(i + pos.manhattan_distance(next) as usize)
                    })
                })
                .filter(|&cost| cost > 0)
                .filter(|&cost| cost >= min_gain)
        })
        .count()
}

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day20, part1)]
fn part1(race_track: &Vec<Vec<char>>) -> usize {
    get_possible_cheat_gains(race_track, 100, 2)
}

#[aoc(day20, part2)]
fn part2(race_track: &Vec<Vec<char>>) -> usize {
    get_possible_cheat_gains(race_track, 100, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part1() {
        assert_eq!(
            get_possible_cheat_gains(&input_generator(EXAMPLE_INPUT), 0, 2),
            44
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            get_possible_cheat_gains(&input_generator(EXAMPLE_INPUT), 50, 20),
            285
        );
    }
}
