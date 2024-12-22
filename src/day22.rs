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
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.secret = evolve_secret(self.secret);
        Some(self.secret)
    }
}

#[aoc(day22, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| PseudoRandom {
            secret: line.parse().unwrap(),
        })
        .map(|mut prng| prng.nth(N - 1).unwrap())
        .sum()
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
            assert_eq!(secret, expected);
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
}
