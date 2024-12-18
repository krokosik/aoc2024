use itertools::Itertools;

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    pointer: usize,
    instructions: Vec<u8>,
}

impl Computer {
    fn combo_operand(&self, value: u8) -> u64 {
        match value {
            0 | 1 | 2 | 3 => value as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => 1,
            _ => panic!("Invalid combo operand"),
        }
    }
}

impl Iterator for Computer {
    type Item = Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer >= self.instructions.len() {
            return None;
        }

        let instruction = Instruction::from(self.instructions[self.pointer]);
        let literal = self.instructions[self.pointer + 1];
        let combo = self.combo_operand(literal);

        self.pointer += 2;

        match instruction {
            Instruction::Adv => self.a = self.a / (1 << combo),
            Instruction::Bdv => self.b = self.a / (1 << combo),
            Instruction::Cdv => self.c = self.a / (1 << combo),
            Instruction::Bxl => self.b ^= literal as u64,
            Instruction::Bxc => self.b ^= self.c,
            Instruction::Bst => self.b = combo % 8,
            Instruction::Jnz => {
                if self.a != 0 {
                    self.pointer = literal as usize;
                }
            }
            Instruction::Out => return Some(Some((combo % 8) as u8)),
        }

        Some(None)
    }
}

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Computer {
    let mut lines_iter = input.lines();

    let a = lines_iter.next().unwrap()[12..].parse().unwrap();
    let b = lines_iter.next().unwrap()[12..].parse().unwrap();
    let c = lines_iter.next().unwrap()[12..].parse().unwrap();

    lines_iter.next();

    let instructions = lines_iter.next().unwrap()[9..]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    Computer {
        a,
        b,
        c,
        pointer: 0,
        instructions,
    }
}

#[aoc(day17, part1)]
fn part1(computer: &Computer) -> String {
    let computer = computer.clone();

    computer.filter_map(|x| x).join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&input_generator(EXAMPLE_INPUT)),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }
}
