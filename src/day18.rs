use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use itertools::{repeat_n, Itertools};

use crate::utils::{Direction, Pos};

fn create_map(size: usize) -> Vec<Vec<char>> {
    (0..size + 2)
        .map(|i| {
            if i == 0 || i == size + 1 {
                vec!['#'; size + 2]
            } else {
                let mut row = Vec::with_capacity(size + 2);
                row.push('#');
                row.extend(repeat_n('.', size));
                row.push('#');
                row
            }
        })
        .collect()
}

fn fall_bytes(input: &str, memory: &mut Vec<Vec<char>>, n: usize) {
    for (x, y) in input.lines().take(n).map(|l| {
        l.split(",")
            .map(|c| c.parse::<usize>().unwrap() + 1)
            .collect_tuple()
            .unwrap()
    }) {
        memory[y][x] = '#';
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    pos: Pos,
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

fn get_dist(dist: &HashMap<Pos, u64>, pos: Pos) -> u64 {
    *dist.get(&pos).unwrap_or(&u64::MAX)
}

fn dijkstra(memory: &Vec<Vec<char>>, start_pos: Pos, end_pos: Pos, dist: &mut HashMap<Pos, u64>) {
    let mut heap = BinaryHeap::new();

    dist.insert(start_pos, 0);

    heap.push(State {
        cost: 0,
        pos: start_pos,
    });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == end_pos {
            break;
        }

        if cost > get_dist(dist, pos) {
            continue;
        }

        for next in Direction::into_iter().map(|dir| State {
            cost: cost + 1,
            pos: pos + dir,
        }) {
            if memory[next.pos] != '#' && next.cost < get_dist(dist, next.pos) {
                dist.insert(next.pos, next.cost);
                heap.push(next);
            }
        }
    }
}

fn find_path_through_bytes(input: &str, size: usize, n: usize) -> u64 {
    let mut memory = create_map(size);
    let start_pos = Pos { x: 1, y: 1 };
    let end_pos = Pos {
        x: size as i64,
        y: size as i64,
    };

    fall_bytes(input, &mut memory, n);

    // for row in &memory {
    //     for &field in row {
    //         print!("{}", field);
    //     }
    //     println!();
    // }

    let mut dist = HashMap::new();
    dijkstra(&memory, start_pos, end_pos, &mut dist);

    get_dist(&dist, end_pos)
}

#[aoc(day18, part1)]
fn part1(input: &str) -> u64 {
    find_path_through_bytes(input, 71, 1024)
}

#[aoc(day18, part2)]
fn part2(input: &str) -> String {
    let mut left = 1024;
    let mut right = input.lines().count();

    while left < right {
        let mid = (left + right) / 2;
        if find_path_through_bytes(input, 71, mid) == u64::MAX {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    input.lines().nth(left - 1).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            find_path_through_bytes(
                "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
                7,
                12
            ),
            22
        );
    }
}
