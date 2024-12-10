use std::{collections::HashSet, iter::repeat_n};

use itertools::Itertools;

type TopographyMap = Vec<Vec<i8>>;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> TopographyMap {
    input
        .lines()
        .map(|line| line.chars().map(|c| (c as i32 - 0x30) as i8).collect())
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &TopographyMap) -> usize {
    let mut visited_points = HashSet::new();

    (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(|&(i, j)| input[i][j] == 0)
        .map(|(i, j)| {
            visited_points.clear();
            calculate_score_1(input, i, j, &mut visited_points)
        })
        .sum()
}

fn calculate_score_1(
    topography_map: &TopographyMap,
    i: usize,
    j: usize,
    visited_points: &mut HashSet<[usize; 2]>,
) -> usize {
    if visited_points.contains(&[i, j]) {
        return 0;
    }
    visited_points.insert([i, j]);

    if topography_map[i][j] == 9 {
        1
    } else {
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .map(|(di, dj)| (i as isize + di, j as isize + dj))
            .filter(|(ni, _)| *ni >= 0)
            .filter(|(_, nj)| *nj >= 0)
            .filter(|(ni, _)| *ni < topography_map.len() as isize)
            .filter(|(_, nj)| *nj < topography_map[i].len() as isize)
            .filter(|&(ni, nj)| {
                topography_map[ni as usize][nj as usize]
                    .overflowing_sub(topography_map[i][j])
                    .0
                    == 1
            })
            .map(|(ni, nj)| {
                calculate_score_1(topography_map, ni as usize, nj as usize, visited_points)
            })
            .sum()
    }
}

#[aoc(day10, part2)]
fn part2(input: &TopographyMap) -> usize {
    let mut scores = repeat_n(repeat_n(0, input[0].len()).collect_vec(), input.len()).collect_vec();

    (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(|&(i, j)| input[i][j] == 0)
        .map(|(i, j)| calculate_score_2(input, i, j, &mut scores))
        .sum()
}

fn calculate_score_2(
    topography_map: &TopographyMap,
    i: usize,
    j: usize,
    scores: &mut Vec<Vec<usize>>,
) -> usize {
    if scores[i][j] != 0 {
        return scores[i][j];
    } else if topography_map[i][j] == 9 {
        scores[i][j] = 1;
        return 1;
    }

    scores[i][j] = [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .iter()
        .map(|(di, dj)| (i as isize + di, j as isize + dj))
        .filter(|(ni, _)| *ni >= 0)
        .filter(|(_, nj)| *nj >= 0)
        .filter(|(ni, _)| *ni < topography_map.len() as isize)
        .filter(|(_, nj)| *nj < topography_map[i].len() as isize)
        .filter(|&(ni, nj)| {
            topography_map[ni as usize][nj as usize]
                .overflowing_sub(topography_map[i][j])
                .0
                == 1
        })
        .map(|(ni, nj)| calculate_score_2(topography_map, ni as usize, nj as usize, scores))
        .sum();

    scores[i][j]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 81);
    }

    #[test]
    fn test_part_simple() {
        assert_eq!(part2(&input_generator("0123456789\n1234567899")), 10);
    }

    #[test]
    fn test_part_simple_2() {
        assert_eq!(
            part2(&input_generator(
                "9990999\n9991998\n9992997\n6543456\n7659987\n8769919\n9879999"
            )),
            13
        );
    }
}
