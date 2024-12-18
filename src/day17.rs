use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
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

    fn walk_back(&mut self) {
        let mut outputs = self.instructions.clone();
        self.pointer -= 2;
        let mut jump_position = self.instructions.len() - 2;

        loop {
            let instruction = Instruction::from(self.instructions[self.pointer]);
            let literal = self.instructions[self.pointer + 1];

            let combo = self.combo_operand(literal);

            match instruction {
                // Some information probably lost here
                Instruction::Adv => self.a <<= combo,
                Instruction::Bdv => self.b <<= combo,
                Instruction::Cdv => self.c <<= combo,
                Instruction::Bxl => self.b ^= literal as u64,
                Instruction::Bxc => self.b ^= self.c,
                Instruction::Bst => match literal {
                    4 => self.a = (self.a & !7) | (self.b & 7),
                    5 => self.b = (self.b & !7) | (self.b & 7),
                    6 => self.c = (self.c & !7) | (self.b & 7),
                    _ => {}
                },
                Instruction::Jnz => {
                    jump_position = self.pointer;
                }
                Instruction::Out => {
                    let out = outputs.pop().unwrap() as u64;
                    match literal {
                        4 => self.a = (self.a & !7) | out,
                        5 => self.b = (self.b & !7) | out,
                        6 => self.c = (self.c & !7) | out,
                        _ => panic!("Invalid output"),
                    }
                }
            }

            if self.pointer == 0 {
                if outputs.is_empty() {
                    break;
                } else {
                    self.pointer = jump_position;
                }
            } else {
                self.pointer -= 2;
            }
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
            Instruction::Adv => self.a = self.a >> combo,
            Instruction::Bdv => self.b = self.a >> combo,
            Instruction::Cdv => self.c = self.a >> combo,
            Instruction::Bxl => self.b ^= literal as u64,
            Instruction::Bxc => self.b ^= self.c,
            Instruction::Bst => self.b = combo & 7,
            Instruction::Jnz => {
                if self.a != 0 {
                    self.pointer = literal as usize;
                }
            }
            Instruction::Out => return Some(Some((combo & 7) as u8)),
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

#[aoc(day17, part2, general)]
fn part2(computer: &Computer) -> u64 {
    // ! There is a bug in the code, so the output is not correct
    let mut computer = computer.clone();

    computer.a = 0;
    computer.b = 0;
    computer.c = 0;
    computer.pointer = computer.instructions.len();
    computer.walk_back();
    computer.a
}

#[aoc(day17, part2, specific)]
fn part2_specific(computer: &Computer) -> u64 {
    let mut minimal_a = 8u64 << (computer.instructions.len() - 1);

    for digit in (0..computer.instructions.len()).rev() {
        let step = 1 << digit * 3;

        while !check_digits(minimal_a, digit, &computer.instructions) {
            minimal_a += step;
        }
    }

    minimal_a
}

fn check_digits(mut a: u64, digit: usize, prog: &Vec<u8>) -> bool {
    a = a >> digit * 3;
    for i in digit..prog.len() {
        let val = ((((a & 7) ^ 1) ^ (a >> ((a & 7) ^ 1))) ^ 6) & 7;
        if prog[i] as u64 != val {
            return false;
        }
        a >>= 3;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_tests() {
        assert_eq!((0b11111 & !7) | 0b101, 0b11101);
        assert_eq!((0b11000 & !7) | 0b101, 0b11101);
        assert_eq!((0b10010 & !7) | 0b101, 0b10101);

        assert_eq!(0b11111u64.trailing_zeros() - 0b11000u64.trailing_zeros(), 3);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(&input_generator(
                "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            )),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(&input_generator(
                "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            )),
            117440
        );
        assert_eq!(
            part1(&input_generator(
                "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            )),
            "0,3,5,4,3,0"
        );
    }
}
