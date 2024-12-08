use itertools::Itertools;
use std::cmp::min;
type Input = Vec<String>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    let n_rows = input.len();
    let n_cols = input[0].len();

    input
        .iter()
        .map(find_xmas)
        .chain((0..n_cols).map(|i| {
            find_xmas(
                &input
                    .iter()
                    .map(|row| row.chars().nth(i).unwrap())
                    .collect(),
            )
        }))
        .chain(
            (1 - n_rows as i32..n_cols as i32)
                .map(|offset| {
                    if offset > 0 {
                        [0, offset as usize]
                    } else {
                        [-offset as usize, 0]
                    }
                })
                .map(|[row_offset, col_offset]| {
                    (0..min(n_rows - row_offset, n_cols - col_offset))
                        .map(|i| input[i + row_offset].chars().nth(i + col_offset).unwrap())
                        .collect()
                })
                .map(|input| find_xmas(&input)),
        )
        .chain(
            (0..n_rows + n_cols - 1)
                .map(|offset| {
                    if offset < n_rows {
                        [offset, 0]
                    } else {
                        [n_rows - 1, offset - n_rows + 1]
                    }
                })
                .map(|[row_offset, col_offset]| {
                    (0..min(row_offset + 1, n_cols - col_offset))
                        .map(|i| input[row_offset - i].chars().nth(col_offset + i).unwrap())
                        .collect()
                })
                .map(|input| find_xmas(&input)),
        )
        .sum()
}

fn _get_column<'a, T>(arr_2d: &'a [&'a [T]]) -> Vec<&'a T>
where
    T: Copy,
{
    (0..arr_2d.len())
        .map(|i| {
            arr_2d
                .iter()
                .flat_map(|row| row.iter().skip(i))
                .next()
                .unwrap()
        })
        .collect()
}


fn find_xmas(str: &String) -> usize {
    str.match_indices("XMAS").count() + str.match_indices("SAMX").count()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
    input[1..input.len() - 1].iter().enumerate().map(|(i, row)| {
        (1..row.len() - 1)
            .map(|j| {
                check_x_mas(&[
                    &input[i][j - 1..j + 2],
                    &input[i + 1][j - 1..j + 2],
                    &input[i + 2][j - 1..j + 2],
                ])
            })
            .sum::<usize>()
    }).sum()
}

fn check_x_mas(input: &[&str]) -> usize {
    if input[1].chars().nth(1) != Some('A') {
        0
    } else {
        let x: Vec<_> = [0, 2]
            .iter()
            .cartesian_product([0, 2].iter())
            .map(|(&i, &j)| input[i].chars().nth(j).unwrap())
            .collect();

        if is_ms(x[0], x[3]) && is_ms(x[1], x[2]) {
            1
        } else {
            0
        }
    }
}

fn is_ms(a: char, b: char) -> bool {
    (a == 'M' && b == 'S') || (a == 'S' && b == 'M')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple() {
        assert_eq!(part1(&input_generator("XMASXMASXMAS")), 3);
    }

    #[test]
    fn horizontal() {
        assert_eq!(part1(&input_generator("advdsXMASsad\nvfdsvdSAMXfdsf")), 2);
    }

    #[test]
    fn vertical() {
        assert_eq!(part1(&input_generator("X\nM\nA\nS\nX\nM\nA\nS")), 2);
    }

    #[test]
    fn diagonal() {
        assert_eq!(part1(&input_generator("X...\n.M..\n..A.\n...S")), 1);
    }

    #[test]
    fn anti_diagonal() {
        assert_eq!(part1(&input_generator(",.,X\n.,M,\n,A,.\nS,.,")), 1);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX")), 18);
    }

    #[test]
    fn simple_cross() {
        assert_eq!(part2(&input_generator("M.S\n.A.\nM.S")), 1);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_generator("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX")), 9);
    }
}
