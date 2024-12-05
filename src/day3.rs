use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<u64>().unwrap();
            let b = cap[2].parse::<u64>().unwrap();
            a * b
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)o\(\)|do(n)'t\(\)").unwrap();
    re.captures_iter(input)
        .fold((1, 0), |(enabled, sum), cap| {
            if let Some(_) = cap.get(3) {
                return (1, sum);
            } else if let Some(_) = cap.get(4) {
                return (0, sum);
            } else {
                let a = cap[1].parse::<u64>().unwrap();
                let b = cap[2].parse::<u64>().unwrap();
                (enabled, sum + a * b * enabled)
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
