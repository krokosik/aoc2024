use itertools::Itertools;

#[derive(Clone, Copy)]
struct LogicalBlock {
    id: Option<usize>,
    length: usize,
}

impl PartialEq for LogicalBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone)]
struct LogicalBlockIterator<'a>(&'a LogicalBlock, usize);

impl<'a> Iterator for LogicalBlockIterator<'a> {
    type Item = Option<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == self.0.length {
            None
        } else {
            self.1 += 1;
            Some(self.0.id)
        }
    }
}

impl<'a> DoubleEndedIterator for LogicalBlockIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'a> IntoIterator for &'a LogicalBlock {
    type Item = Option<usize>;
    type IntoIter = LogicalBlockIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LogicalBlockIterator(self, 0)
    }
}

type DiskMap = Vec<LogicalBlock>;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> DiskMap {
    input
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| c as i32 - 0x30)
        .enumerate()
        .map(|(i, block_size)| LogicalBlock {
            id: if i % 2 == 0 { Some(i / 2) } else { None },
            length: block_size as usize,
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &DiskMap) -> usize {
    let simplified_input = input.iter().flat_map(|block| block.into_iter());

    let mut rev_iter = simplified_input.clone().flatten().rev();

    simplified_input
        .clone()
        .take(simplified_input.clone().flatten().count())
        .map(|block| match block {
            Some(id) => id,
            None => rev_iter.next().unwrap(),
        })
        .enumerate()
        .map(|(i, id)| i * id)
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &DiskMap) -> usize {
    let mut ids_to_skip: Vec<usize> = vec![];

    input
        .iter()
        .enumerate()
        .map(|(i, logic_block)| match logic_block.id {
            Some(id) => {
                if ids_to_skip.contains(&id) {
                    vec![LogicalBlock {
                        id: None,
                        length: logic_block.length,
                    }]
                } else {
                    vec![*logic_block]
                }
            }
            None => {
                let mut length_remaining = logic_block.length;
                let mut blocks = vec![];

                input
                    .iter()
                    .skip(i + 1)
                    .rev()
                    .skip_while(|logic_block| logic_block.id.is_none())
                    .step_by(2)
                    .for_each(|logic_block| {
                        if length_remaining < logic_block.length
                            || ids_to_skip.contains(&logic_block.id.unwrap())
                        {
                            return;
                        }

                        length_remaining -= logic_block.length;

                        blocks.push(*logic_block);
                        ids_to_skip.push(logic_block.id.unwrap());
                    });

                blocks.push(LogicalBlock {
                    id: None,
                    length: length_remaining,
                });

                blocks
            }
        })
        .flatten()
        .flat_map(|logical_block| logical_block.into_iter().collect_vec())
        .map(|block| match block {
            Some(id) => id,
            None => 0,
        })
        .enumerate()
        .map(|(i, id)| i * id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 2858);
    }
}
