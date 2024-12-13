use itertools::Itertools;

const CARDINAL_DIRECTIONS: [[isize; 2]; 4] = [[-1, 0], [0, -1], [1, 0], [0, 1]];

#[aoc(day12, part1)]
fn part1(input: &str) -> u64 {
    let mut map = input
        .lines()
        .map(|line| line.chars().map(Some).collect_vec())
        .collect_vec();
    let mut res = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if let Some(region) = map[i][j] {
                res += crawl_region(&mut map, i, j, region);
            }
        }
    }

    res
}

fn crawl_region(map: &mut Vec<Vec<Option<char>>>, i: usize, j: usize, region: char) -> u64 {
    let mut stack = vec![(i, j)];
    let mut visited = vec![(i, j)];
    let mut area = 0;
    let mut perimeter = 0;

    while let Some((i, j)) = stack.pop() {
        area += 1;
        for &[di, dj] in CARDINAL_DIRECTIONS.iter() {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if !(ni >= 0
                && nj >= 0
                && ni < map.len() as isize
                && nj < map[i].len() as isize
                && map[ni as usize][nj as usize] == Some(region))
            {
                perimeter += 1;
            } else {
                if !visited.contains(&(ni as usize, nj as usize)) {
                    stack.push((ni as usize, nj as usize));
                    visited.push((ni as usize, nj as usize));
                }
            }
        }
    }

    for (i, j) in visited {
        map[i][j] = None;
    }

    area * perimeter
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u64 {
    let mut map = input
        .lines()
        .map(|line| line.chars().map(Some).collect_vec())
        .collect_vec();
    let mut res = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if let Some(region) = map[i][j] {
                res += crawl_region_with_discount(&mut map, i, j, region);
            }
        }
    }

    res
}

fn crawl_region_with_discount(
    map: &mut Vec<Vec<Option<char>>>,
    i: usize,
    j: usize,
    region: char,
) -> u64 {
    let mut stack = vec![(i, j)];
    let mut visited = vec![(i, j)];
    let mut area = 0;
    let mut vertices = 0;

    while let Some((i, j)) = stack.pop() {
        area += 1;

        let all_neighbours = [-1, 0, 1]
            .iter()
            .flat_map(|&dj| {
                [-1, 0, 1]
                    .iter()
                    .map(move |&di| (i as isize + di, j as isize + dj))
            })
            .map(|(ni, nj)| {
                if ni >= 0 && nj >= 0 && ni < map.len() as isize && nj < map[i].len() as isize {
                    map[ni as usize][nj as usize]
                } else {
                    None
                }
            })
            .collect_vec();

        for neighbour_idx in (1..all_neighbours.len()).step_by(2) {
            let triplet = (match neighbour_idx {
                1 => [1, 0, 3],
                3 => [3, 6, 7],
                5 => [5, 2, 1],
                7 => [7, 8, 5],
                _ => panic!("Invalid neighbour index {}", neighbour_idx),
            })
            .iter()
            .map(|&idx| all_neighbours[idx] == Some(region))
            .collect_vec();

            match triplet[0..3] {
                [true, false, true] | [false, _, false] => vertices += 1,
                _ => {}
            }
            let ni = (i as isize + ((neighbour_idx as isize) % 3 - 1)) as usize;
            let nj = (j as isize + (neighbour_idx as isize) / 3 - 1) as usize;
            if triplet[0] && ni < map.len() && nj < map[i].len() && !visited.contains(&(ni, nj)) {
                stack.push((ni, nj));
                visited.push((ni, nj));
            }
        }
    }

    for (i, j) in visited {
        map[i][j] = None;
    }

    area * vertices
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = "AAAA\nBBCD\nBBCC\nEEEC";
    const EXAMPLE_INPUT: &str = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";

    #[test]
    fn part1_simplest_example() {
        assert_eq!(part1(SIMPLE_INPUT), 140);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 1930);
    }

    #[test]
    fn part2_simplest_example() {
        assert_eq!(part2(SIMPLE_INPUT), 80);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 1206);
    }
}
