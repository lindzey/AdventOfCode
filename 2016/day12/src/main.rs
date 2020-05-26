use std::collections::HashMap;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::*;
        use pretty_assertions::assert_eq;

        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let instructions = load_instructions(&input);
        let mut computer = Computer::new();
        computer.run_instructions(&instructions);
        assert_eq!(computer.registers[&'a'], 42);
    }
}

struct Computer {
    registers: HashMap<char, i32>,
}

impl Computer {
    fn new() -> Computer {
        let mut registers = HashMap::new();
        for ch in &['a', 'b', 'c', 'd'] {
            registers.insert(*ch, 0);
        }
        Computer {
            registers
        }
    }

    fn run_instructions(&mut self, instructions: &[Instruction]) {
        let mut idx = 0;
        while idx < instructions.len() {
            match &instructions[idx] {
                Instruction::Cpy(val, reg) => {
                    match val {
                        Value::Register(rr) => *self.registers.get_mut(reg).unwrap() = *self.registers.get(rr).unwrap(),
                        Value::Number(ii) => *self.registers.get_mut(reg).unwrap() = *ii,
                    };
                    idx += 1;
                },
                Instruction::Inc(reg) => {
                    *self.registers.get_mut(reg).unwrap() += 1;
                    idx += 1;
                },
                Instruction::Dec(reg) => {
                    *self.registers.get_mut(reg).unwrap() -= 1;
                    idx += 1;
                },
                Instruction::Jnz(val, jmp) => {
                    match val {
                        Value::Register(reg) => {
                            if 0 != *self.registers.get_mut(reg).unwrap() {
                                idx = (idx as i32 + jmp) as usize;
                            } else {
                                idx += 1;
                            }
                        },
                        Value::Number(ii) => {
                            if 0 != *ii {
                                idx = (idx as i32 + jmp) as usize;
                            } else {
                                idx += 1;
                            }
                        },
                    };
                },
            }
        }
    }
}

enum Instruction {
  Cpy(Value, char),
  Inc(char),
  Dec(char),
  Jnz(Value, i32),
}

enum Value{
    Register(char),
    Number(i32),
}

impl Value {
    fn from(input: &str) -> Value {
        let parsed = input.parse::<i32>();
        if let Ok(num) = parsed {
            Value::Number(num)
        } else {
            Value::Register(input.chars().next().unwrap())
        } 
    }
}

impl Instruction {
    /*
    cpy x y copies x (either an integer or the value of a register) into register y.
    inc x increases the value of register x by one.
    dec x decreases the value of register x by one.
    jnz x y jumps to an instruction y away (positive means forward; negative means backward), but only if x is not zero.
    */
    fn from(input: &str) -> Instruction {
        let tokens: Vec<&str> = input.split_whitespace().collect();
        match tokens[0] {
            "cpy" => {
                Instruction::Cpy(Value::from(tokens[1]), tokens[2].chars().next().unwrap())
            },
            "inc" => {
                Instruction::Inc(tokens[1].chars().next().unwrap())
            },
            "dec" => {
                Instruction::Dec(tokens[1].chars().next().unwrap())
            },
            "jnz" => {
                Instruction::Jnz(Value::from(tokens[1]), tokens[2].parse::<i32>().unwrap())
            },
            _ => panic!("Unrecognized instruction: {}", input),
        }
    }
}

// TODO: It seems like there should be a more functional way to do this?
fn load_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(Instruction::from(line));
    }
    instructions
}

fn part1(input: &str) -> i32 {
    let instructions = load_instructions(&input);
    let mut computer = Computer::new();
    computer.run_instructions(&instructions);
    computer.registers[&'a']
}

fn part2(input: &str) -> i32 {
    let instructions = load_instructions(&input);
    let mut computer = Computer::new();
    *computer.registers.get_mut(&'c').unwrap() = 1;
    computer.run_instructions(&instructions);
    computer.registers[&'a']
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1); 
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2); 
}
