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

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let race_track = input.lines().map(|line| line.chars().collect()).collect();

    let path = get_path(&race_track);

    let costs = path
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i))
        .collect::<HashMap<_, _>>();

    path.iter()
        .enumerate()
        .flat_map(|(i, pos)| {
            Direction::into_iter()
                .map(|dir| costs.get(&(*pos + dir + dir)).unwrap_or(&0))
                .map(move |&cost| cost.checked_sub(i + 2).unwrap_or(0))
                // .filter(|&cost| cost >= 100)
                .filter(|&cost| cost > 0)
        })
        .count()
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
        assert_eq!(part1(EXAMPLE_INPUT), 44);
    }
}
