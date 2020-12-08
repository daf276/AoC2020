use itertools::Itertools;

#[derive(Copy, Clone)]
pub enum Instruction {
    NOP,
    ACC,
    JMP,
}

#[derive(Copy, Clone)]
pub struct OP {
    instruction: Instruction,
    value: isize,
}

pub fn read_input() -> Vec<OP> {
    return include_str!("../input/day8")
        .lines()
        .map(|line| parse_line(line))
        .collect();
}

fn parse_line(line: &str) -> OP {
    let (instruction_str, value_str) = line.split(" ").collect_tuple().unwrap();
    let instruction = match instruction_str {
        "jmp" => Instruction::JMP,
        "nop" => Instruction::NOP,
        "acc" => Instruction::ACC,
        _ => unreachable!(),
    };
    let value = value_str.parse::<isize>().unwrap();

    return OP { instruction, value };
}

pub fn part1(input: &Vec<OP>) -> isize {
    let (acc, _) = emulate(input);
    return acc;
}

pub fn part2(input: &Vec<OP>) -> isize {
    for i in 0..input.len() {
        let mut new_ops = input.to_vec();
        match new_ops[i].instruction {
            Instruction::ACC => continue,
            Instruction::NOP => new_ops[i].instruction = Instruction::JMP,
            Instruction::JMP => new_ops[i].instruction = Instruction::NOP,
        }

        let (acc, terminated) = emulate(&new_ops);
        if terminated {
            return acc;
        }
    }

    return 0;
}

//Returns the accumulator and whether the program terminated normally
fn emulate(ops: &Vec<OP>) -> (isize, bool) {
    let mut reached = vec![false; ops.len()];
    let mut acc = 0;
    let mut i = 0;

    while i < ops.len() as isize {
        if reached[i as usize] {
            return (acc, false);
        } else {
            reached[i as usize] = true;
            match ops[i as usize].instruction {
                Instruction::NOP => i += 1,
                Instruction::JMP => i += ops[i as usize].value,
                Instruction::ACC => {
                    acc += ops[i as usize].value;
                    i += 1
                }
            }
        }
    }

    return (acc, true);
}
