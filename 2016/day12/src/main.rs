use std::collections::HashMap;
use std::error::Error;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::*;
        use pretty_assertions::assert_eq;

        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let instructions = load_program(&input).unwrap();
        let mut computer = Computer::new();
        computer.run_program(&instructions);
        assert_eq!(computer.get_reg(&Register::A), 42);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Default)]
struct Computer {
    registers: HashMap<Register, i32>,
}

impl Computer {
    fn new() -> Computer {
        let mut registers = HashMap::new();
        use Register::*;
        // TODO: Really, I wanted to automatically insert every register into 
        // the computer so key-not-found errors would be impossible.
        // However, it doesn't seem like there's a way (in the standard
        // library) to iterate over static enums.
        for reg in &[A, B, C, D] {
            registers.insert(*reg, 0);
        }
        Computer {
            registers
        }
    }

    fn get_reg(&self, reg: &Register) -> i32 {
        *self.registers.get(reg).unwrap()
    }

    fn set_reg(&mut self, reg: &Register, val: i32) {
        *self.registers.get_mut(reg).unwrap() = val;
    }

    fn run_program(&mut self, instructions: &Program) {
        let mut idx = 0;
        while idx < instructions.len() {
            match &instructions[idx] {
                Instruction::Cpy(val, dest_reg) => {
                    match val {
                        Operand::Register(src_reg) => self.set_reg(dest_reg, self.get_reg(src_reg)),
                        Operand::Number(ii) => self.set_reg(dest_reg, *ii),
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
                        Operand::Register(reg) => {
                            if 0 != *self.registers.get_mut(reg).unwrap() {
                                idx = (idx as i32 + jmp) as usize;
                            } else {
                                idx += 1;
                            }
                        },
                        Operand::Number(ii) => {
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
  Cpy(Operand, Register),
  Inc(Register),
  Dec(Register),
  Jnz(Operand, i32),
}

enum Operand {
    Register(Register),
    Number(i32),
}

#[derive(Debug)]
struct AssemBunnyParseError {
    message: String,
}
impl AssemBunnyParseError {
    fn new(msg: &str) -> AssemBunnyParseError {
        AssemBunnyParseError{message: msg.to_string()} 
    }
}
impl std::fmt::Display for AssemBunnyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for AssemBunnyParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Register {
    fn from(input: &str) -> Result<Register, AssemBunnyParseError> {
        match input {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(AssemBunnyParseError::new(&format!("Unable to parse {} into a Register", input)))
        }
    }
}

impl Operand {
    fn from(input: &str) -> Result<Operand, AssemBunnyParseError> {
        let parsed = input.parse::<i32>();
        if let Ok(num) = parsed {
            Ok(Operand::Number(num))
        } else {
            match Register::from(input) {
                Ok(reg) => Ok(Operand::Register(reg)),
                Err(err) => Err(err),
            }
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
    fn from(input: &str) -> Result<Instruction, AssemBunnyParseError> {
        let tokens: Vec<&str> = input.split_whitespace().collect();
        match tokens[0] {
            "cpy" => {
                Ok(Instruction::Cpy(Operand::from(tokens[1])?, Register::from(tokens[2])?))
            },
            "inc" => {
                Ok(Instruction::Inc(Register::from(tokens[1])?))
            },
            "dec" => {
                Ok(Instruction::Dec(Register::from(tokens[1])?))
            },
            "jnz" => {
                // TODO: Figure out how to Box my error s.t. I can also pass up the i32 parse error...
                Ok(Instruction::Jnz(Operand::from(tokens[1])?, tokens[2].parse::<i32>().unwrap()))
            },
            _ => panic!("Unrecognized instruction: {}", input),
        }
    }
}

// TODO: It seems like there should be a more functional way to do this?
type Program = Vec<Instruction>;
fn load_program(input: &str) -> Result<Program, AssemBunnyParseError> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(Instruction::from(line)?);
    }
    Ok(instructions)
}

fn part1(input: &str) -> i32 {
    let instructions = load_program(&input).unwrap();
    let mut computer = Computer::new();
    computer.run_program(&instructions);
    computer.get_reg(&Register::A)
}

fn part2(input: &str) -> i32 {
    let instructions = load_program(&input).unwrap();
    let mut computer = Computer::new();
    *computer.registers.get_mut(&Register::C).unwrap() = 1;
    computer.run_program(&instructions);
    computer.get_reg(&Register::A)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1); 
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2); 
}
