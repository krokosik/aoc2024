use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

type Wires<'a> = HashMap<&'a str, bool>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum GateType {
    AND,
    OR,
    XOR,
}

impl From<&str> for GateType {
    fn from(s: &str) -> Self {
        match s {
            "AND" => GateType::AND,
            "OR" => GateType::OR,
            "XOR" => GateType::XOR,
            _ => panic!("Invalid gate type"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    output: &'a str,
    gate_type: GateType,
}

type GateQueue<'a> = VecDeque<Gate<'a>>;

fn input_generator<'a>(input: &'a str) -> (Wires<'a>, GateQueue<'a>) {
    let mut wires = HashMap::new();
    let mut gates = VecDeque::new();

    let mut lines = input.lines();

    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let mut line = line.split(": ");
            let wire = line.next().unwrap();
            let value = line.next().unwrap().parse::<u8>().unwrap() == 1;
            wires.insert(wire, value);
        });

    lines.for_each(|line| {
        let mut line = line.split(" -> ");
        let inputs = line.next().unwrap();
        let output = line.next().unwrap();

        let mut inputs = inputs.split(" ");
        let input1 = inputs.next().unwrap();
        let gate_type = inputs.next().unwrap();
        let input2 = inputs.next().unwrap();

        gates.push_back(Gate {
            input1,
            input2,
            output,
            gate_type: GateType::from(gate_type),
        });
    });

    (wires, gates)
}

fn wires_to_numbers(wires: &Wires, letter: char) -> u64 {
    wires
        .keys()
        .filter(|&&key| key.starts_with(letter) && wires[key])
        .fold(0, |acc, key| acc | 1 << key[1..].parse::<u64>().unwrap())
}

fn produces_output<'a>(wires: &'a mut Wires<'a>, gates: &'a mut GateQueue<'a>) -> &'a Wires<'a> {
    while let Some(gate) = gates.pop_front() {
        let input1 = match wires.get(gate.input1) {
            Some(value) => *value,
            None => {
                gates.push_back(gate);
                continue;
            }
        };

        let input2 = match wires.get(gate.input2) {
            Some(value) => *value,
            None => {
                gates.push_back(gate);
                continue;
            }
        };

        let output = match gate.gate_type {
            GateType::AND => input1 & input2,
            GateType::OR => input1 | input2,
            GateType::XOR => input1 ^ input2,
        };

        wires.insert(gate.output, output);
    }

    wires
}

#[aoc(day24, part1)]
fn part1(input: &str) -> u64 {
    let (mut wires, mut gates) = input_generator(input);

    let wires = produces_output(&mut wires, &mut gates);

    wires_to_numbers(&wires, 'z')
}

#[aoc(day24, part2)]
fn part2(input: &str) -> String {
    let (wires, gates) = input_generator(input);

    let mut output_swaps = vec![];

    let z_len = wires.keys().filter(|&&key| key.starts_with('x')).count();

    for &Gate {
        input1,
        input2,
        output,
        gate_type,
    } in &gates
    {
        let (input1, input2) = [input1, input2]
            .into_iter()
            .sorted()
            .collect_tuple()
            .unwrap();
        if output.starts_with("z") && !output.ends_with(z_len.to_string().as_str()) {
            if gate_type != GateType::XOR {
                output_swaps.push(output);
            }
        } else if !(input1.starts_with("x") || input2.starts_with("y")) {
            if gate_type == GateType::XOR {
                output_swaps.push(output);
            }
        } else if input1.starts_with("x") && input2.starts_with("y")
            || input1.starts_with("y") && input2.starts_with("x")
        {
            if input1.ends_with("00") || input2.ends_with("00") {
                continue;
            }

            let mut ops = vec![];

            for other_gate in &gates {
                if other_gate.input1 == output || other_gate.input2 == output {
                    ops.push(other_gate.gate_type);
                }
            }

            if gate_type == GateType::XOR && !ops.contains(&GateType::XOR)
                || gate_type == GateType::AND && !ops.contains(&GateType::OR)
            {
                output_swaps.push(output);
            }
        }
    }

    output_swaps.iter().sorted().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const EXAMPLE_INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SMALL_INPUT), 0b100);
        assert_eq!(part1(EXAMPLE_INPUT), 0b0011111101000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"
            ),
            "z00,z01,z02,z05"
        );
    }
}
