use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Input {
    rules_inverse: HashMap<u32, Rc<HashSet<u32>>>,
    updates: Vec<Vec<Page>>,
}

#[derive(Debug, Clone)]
struct Page {
    number: u32,
    predecessors: Rc<HashSet<u32>>,
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for Page {}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other
            .predecessors
            .contains(&self.number)
            .then(|| std::cmp::Ordering::Less)
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let mut rules_inverse: HashMap<u32, HashSet<u32>> = HashMap::new();

    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (left, right) = line
                .split('|')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            if rules_inverse.contains_key(&right) {
                rules_inverse.get_mut(&right).unwrap().insert(left);
            } else {
                rules_inverse.insert(right, HashSet::from([left]));
            }

            if !rules_inverse.contains_key(&left) {
                rules_inverse.insert(left, HashSet::new());
            }
        });

    let rules_inverse: HashMap<u32, Rc<HashSet<u32>>> = rules_inverse
        .into_iter()
        .map(|(k, v)| (k, Rc::new(v)))
        .collect();

    let mut updates = vec![];
    lines.for_each(|line| {
        updates.push(
            line.split(',')
                .map(|s| s.parse().unwrap())
                .map(|number| Page {
                    number,
                    predecessors: rules_inverse.get(&number).unwrap().clone(),
                })
                .collect(),
        );
    });

    Input {
        rules_inverse,
        updates,
    }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .filter(|updates| updates.iter().is_sorted())
        .map(|updates| updates[updates.len() / 2].number)
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .filter(|updates| !updates.iter().is_sorted())
        .map(|updates| updates.iter().sorted().collect::<Vec<_>>())
        .map(|updates| updates[updates.len() / 2].number)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        let rc1 = Rc::new(HashSet::from([2]));
        let rc2 = Rc::new(HashSet::from([1]));
        let rc1_clone = rc1.clone();
        let rc2_clone = rc2.clone();
        assert_eq!(
            input_generator("1|2\n2|1\n\n1,2"),
            Input {
                rules_inverse: vec![(2, rc2), (1, rc1)].into_iter().collect(),
                updates: vec![vec![
                    Page {
                        number: 1,
                        predecessors: rc1_clone
                    },
                    Page {
                        number: 2,
                        predecessors: rc2_clone
                    },
                ]]
            }
        )
    }

    const EXAMPLE_INPUT: &str = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 123);
    }
}
