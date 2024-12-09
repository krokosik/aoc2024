use itertools::Itertools;

#[derive(Clone, Copy)]
struct BlockSegment {
    id: Option<usize>,
    length: usize,
}

impl PartialEq for BlockSegment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone)]
struct BlockSegmentIterator<'a>(&'a BlockSegment, usize);

impl<'a> Iterator for BlockSegmentIterator<'a> {
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

impl<'a> DoubleEndedIterator for BlockSegmentIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'a> IntoIterator for &'a BlockSegment {
    type Item = Option<usize>;
    type IntoIter = BlockSegmentIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockSegmentIterator(self, 0)
    }
}

type DiskMap = Vec<BlockSegment>;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> DiskMap {
    input
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| c as i32 - 0x30)
        .enumerate()
        .map(|(i, segment_size)| BlockSegment {
            id: if i % 2 == 0 { Some(i / 2) } else { None },
            length: segment_size as usize,
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &DiskMap) -> usize {
    let simplified_input = input
        .iter()
        .flat_map(|block_segment| block_segment.into_iter());

    let mut rev_iter = simplified_input.clone().flatten().rev();

    simplified_input
        .clone()
        .take(simplified_input.clone().flatten().count())
        .map(|block_segment| match block_segment {
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
        .flat_map(|(i, block_segment)| match block_segment.id {
            Some(id) => {
                if ids_to_skip.contains(&id) {
                    vec![BlockSegment {
                        id: None,
                        length: block_segment.length,
                    }]
                } else {
                    vec![*block_segment]
                }
            }
            None => {
                let mut length_remaining = block_segment.length;
                let mut blocks = vec![];

                input
                    .iter()
                    .skip(i + 1)
                    .rev()
                    .skip_while(|block_segment| block_segment.id.is_none())
                    .step_by(2)
                    .for_each(|block_segment| {
                        if length_remaining < block_segment.length
                            || ids_to_skip.contains(&block_segment.id.unwrap())
                        {
                            return;
                        }

                        length_remaining -= block_segment.length;

                        blocks.push(*block_segment);
                        ids_to_skip.push(block_segment.id.unwrap());
                    });

                blocks.push(BlockSegment {
                    id: None,
                    length: length_remaining,
                });

                blocks
            }
        })
        .flat_map(|block_segment| block_segment.into_iter().collect_vec())
        .map(|block_segment| match block_segment {
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
