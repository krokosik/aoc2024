use std::collections::HashMap;

use itertools::Itertools;

const MODULUS: u64 = 16777215;
const N: usize = 2000;

fn evolve_secret(secret: u64) -> u64 {
    let secret = (secret ^ (secret << 6)) & MODULUS;
    let secret = (secret ^ (secret >> 5)) & MODULUS;
    let secret = (secret ^ (secret << 11)) & MODULUS;
    secret
}

struct PseudoRandom {
    secret: u64,
}

impl Iterator for PseudoRandom {
    type Item = (u64, u8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        let secret = self.secret;
        let price = secret % 10;
        let new_secret = evolve_secret(secret);
        let new_price = new_secret % 10;
        self.secret = new_secret;
        Some((self.secret, new_price as u8, new_price as i8 - price as i8))
    }
}

#[aoc(day22, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| PseudoRandom {
            secret: line.parse().unwrap(),
        })
        .map(|mut prng| prng.nth(N - 1).unwrap().0)
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &str) -> u64 {
    let price_lists = input
        .lines()
        .map(|line| {
            let secret = line.parse().unwrap();
            let prng = PseudoRandom { secret };
            let mut sequence_price = HashMap::new();

            for window in prng.take(N).collect_vec().windows(4) {
                let delta_sequence = window.iter().map(|(_, _, delta)| *delta).collect_vec();
                let price = window[3].1;
                sequence_price.entry(delta_sequence).or_insert(price);
            }

            sequence_price
        })
        .collect_vec();

    price_lists
        .iter()
        .flat_map(|price_list| price_list.keys())
        .sorted()
        .dedup()
        .map(|sequence| {
            price_lists
                .iter()
                .map(|price_list| *price_list.get(sequence).unwrap_or(&0) as u64)
                .sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let prng = PseudoRandom { secret: 123 };
        for (secret, expected) in std::iter::zip(
            prng.take(10),
            [
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ],
        ) {
            assert_eq!(secret.0, expected);
        }
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "1
10
100
2024"
            ),
            37327623
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "1
2
3
2024"
            ),
            23
        );
    }
}
