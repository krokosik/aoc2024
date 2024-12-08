use std::collections::HashMap;

use itertools::Itertools;

struct Input {
    antennas: HashMap<char, Vec<[usize; 2]>>,
    width: usize,
    height: usize,
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Input {
    let mut antennas = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        height += 1;
        width = line.len();
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => {}
            _ => {
                let entry = antennas.entry(c).or_insert(vec![]);
                entry.push([x, y]);
            }
        });
    });

    Input {
        antennas,
        width,
        height,
    }
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    input
        .antennas
        .values()
        .flat_map(|antenna_positions| {
            antenna_positions.iter().enumerate().flat_map(|(i, ant1)| {
                antenna_positions
                    .iter()
                    .skip(i + 1)
                    .flat_map(|ant2| get_antinodes(ant1, ant2, input.width, input.height, false))
            })
        })
        .sorted()
        .dedup()
        .count()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    input
        .antennas
        .values()
        .filter(|antenna_positions| antenna_positions.len() > 1)
        .flat_map(|antenna_positions| {
            antenna_positions
                .iter()
                .enumerate()
                .flat_map(|(i, ant1)| {
                    antenna_positions
                        .iter()
                        .skip(i + 1)
                        .flat_map(|ant2| get_antinodes(ant1, ant2, input.width, input.height, true))
                })
                .chain(
                    antenna_positions
                        .iter()
                        .map(|&[x, y]| [x as isize, y as isize]),
                )
        })
        .sorted()
        .dedup()
        .count()
}

fn get_antinodes(
    ant1: &[usize; 2],
    ant2: &[usize; 2],
    width: usize,
    height: usize,
    with_harmonics: bool,
) -> Vec<[isize; 2]> {
    let x1 = ant1[0] as isize;
    let y1 = ant1[1] as isize;
    let x2 = ant2[0] as isize;
    let y2 = ant2[1] as isize;

    let offset_x = x2 - x1;
    let offset_y = y2 - y1;

    let res_iter_1 = anti_node_iter(x1, y1, -offset_x, -offset_y, width, height);
    let res_iter_2 = anti_node_iter(x2, y2, offset_x, offset_y, width, height);

    if !with_harmonics {
        res_iter_1.take(1).chain(res_iter_2.take(1)).collect()
    } else {
        res_iter_1.chain(res_iter_2).collect()
    }
}

fn anti_node_iter<'a>(
    x0: isize,
    y0: isize,
    offset_x: isize,
    offset_y: isize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = [isize; 2]> + 'a {
    (1..)
        .map(move |i| [x0 + i * offset_x, y0 + i * offset_y])
        .take_while(move |&[x, y]| x >= 0 && y >= 0 && x < width as isize && y < height as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 14);
    }

    #[test]
    fn simple_2_antenna() {
        assert_eq!(part1(&input_generator("....\n.0..\n..0.\n....")), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 34);
    }

    #[test]
    fn iter_generator() {
        assert_eq!(
            anti_node_iter(0, 0, 1, 1, 3, 3).collect_vec(),
            vec![[1, 1], [2, 2]]
        )
    }

    #[test]
    fn iter_generator_rev() {
        assert_eq!(
            anti_node_iter(2, 2, -1, -1, 3, 3).collect_vec(),
            vec![[1, 1], [0, 0]]
        )
    }
}
