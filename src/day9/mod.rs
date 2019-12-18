use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{hash_map::Entry, HashMap, HashSet};

macro_rules! address_or_value {
    ($iter_name:ident, $cpu:ident, $expression:expr) => {
        match $iter_name.next() {
            Some(val) => {
                match val.to_digit(10).expect("Parameter mode is not a digit!") {
                    0 => {
                        //debug_print!("Position mode!");
                        $cpu.get_value($expression)
                    },
                    1 => {
                        //debug_print!("Immediate mode!");
                        $cpu.get_memory($expression)
                    }, 
                    2 => {
                        //debug_print!("Relative mode!");
                        $cpu.get_relative($expression)
                    },
                    _ => {
                        unreachable!()
                    }
                }
            }
            None => {
                //debug_print!("Position mode!");
                $cpu.get_value($expression)
            }
        }
    };
}

macro_rules! get_location {
    ($iter_name:ident, $cpu:ident, $expression:expr) => {
        match $iter_name.next() {
            Some('2') => {
                $cpu.get_memory($expression) + $cpu.relative_base
            },
            _ => {
                $cpu.get_memory($expression)
            }
        }
    };
}

type CpuResult<T> = std::result::Result<T, CpuError>;

#[derive(Debug, Clone, Copy)]
enum CpuError {
    InvalidOpcode(isize, usize),
    InvalidLastInstruction,
    InvalidUserInput,
}

#[derive(Debug, Clone, Copy)]
enum ExitReason {
    Halt,
    InputRequired,
    OutputGenerated
}

#[derive(Debug, Clone)]
struct CPU {
    memory: Vec<isize>,
    instruction_pointer: usize,
    relative_base: isize,
    last_instruction: Option<Instruction>,
    output: Vec<isize>,

    exit_on_output: bool,
}

impl CPU {
    pub fn new(program: &str) -> CPU {
        CPU {
            memory: program
                .trim()
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect(),
            instruction_pointer: 0,
            relative_base: 0,
            last_instruction: None,
            output: vec![],
            exit_on_output: false,
        }
    }

    pub fn run(&mut self, input: Option<&str>) -> CpuResult<ExitReason> {
        let mut user_input = input.unwrap_or("").trim().lines();
        loop {
            match self.parse()? {
                Instruction::Add(left, right, location) => {
                    debug_print!("Add : {} + {} @ {}", left, right, location);
                    self.last_instruction = Some(Instruction::Add(left, right, location));
                    self.set_memory(location as usize, left + right);
                }
                Instruction::Mult(left, right, location) => {
                    debug_print!("Mult : {} * {} @ {}", left, right, location);
                    self.last_instruction = Some(Instruction::Mult(left, right, location));
                    self.set_memory(location as usize, left * right);
                }
                Instruction::In(location) => {
                    debug_print!("In : @ {}", location);
                    self.last_instruction = Some(Instruction::In(location));
                    self.set_memory(
                        location as usize,
                        if input.is_some() {
                            match user_input.next() {
                                Some(val) => val.trim().parse::<isize>().unwrap(),
                                None => return Ok(ExitReason::InputRequired),
                            }
                        } else {
                            return Ok(ExitReason::InputRequired);
                        },
                    );
                }
                Instruction::Out(value) => {
                    debug_print!("Out : {}", value);
                    self.last_instruction = Some(Instruction::Out(value));
                    self.output.push(value);
                    if self.exit_on_output {
                        self.increment_ip()?;
                        return Ok(ExitReason::OutputGenerated);
                    }
                }
                Instruction::JumpIfTrue(value, new_ip) => {
                    debug_print!("JIT : {} to {}", value, new_ip);
                    self.last_instruction = Some(Instruction::JumpIfTrue(value, new_ip));
                    if value != 0 {
                        self.instruction_pointer = new_ip as usize;
                    }
                }
                Instruction::JumpIfFalse(value, new_ip) => {
                    debug_print!("JIF : {} to {}", value, new_ip);
                    self.last_instruction = Some(Instruction::JumpIfFalse(value, new_ip));
                    if value == 0 {
                        self.instruction_pointer = new_ip as usize;
                    }
                }
                Instruction::LessThan(left, right, location) => {
                    debug_print!("LT : {} < {} @ {}", left, right, location);
                    self.last_instruction = Some(Instruction::LessThan(left, right, location));
                    if left < right {
                        self.set_memory(location as usize, 1);
                    } else {
                        self.set_memory(location as usize, 0);
                    }
                }
                Instruction::Equal(left, right, location) => {
                    debug_print!("EQ : {} == {} @ {}", left, right, location);
                    self.last_instruction = Some(Instruction::Equal(left, right, location));
                    if left == right {
                        self.set_memory(location as usize, 1);
                    } else {
                        self.set_memory(location as usize, 0);
                    }
                }
                Instruction::AdjustRelativeBase(value) => {
                    debug_print!("AdjustRelBase : {} + {}", self.relative_base, value);
                    self.last_instruction = Some(Instruction::AdjustRelativeBase(value));
                    self.relative_base += value;
                }
                Instruction::Halt => {
                    debug_print!("Halt");
                    self.last_instruction = Some(Instruction::Halt);
                    return Ok(ExitReason::Halt);
                }
            }
            self.increment_ip()?;
        }
    }

    fn parse(&self) -> CpuResult<Instruction> {
        Instruction::parse(&self)
    }

    fn increment_ip(&mut self) -> CpuResult<()> {
        if let Some(last_instr) = self.last_instruction {
            match last_instr {
                Instruction::Add(_, _, _) => {
                    self.instruction_pointer += 4;
                }
                Instruction::Mult(_, _, _) => {
                    self.instruction_pointer += 4;
                }
                Instruction::In(_) => {
                    self.instruction_pointer += 2;
                }
                Instruction::Out(_) => {
                    self.instruction_pointer += 2;
                }
                Instruction::JumpIfTrue(value, _) => {
                    if value != 0 {
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Instruction::JumpIfFalse(value, _) => {
                    if value == 0 {
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Instruction::LessThan(_, _, _) => {
                    self.instruction_pointer += 4;
                }
                Instruction::Equal(_, _, _) => {
                    self.instruction_pointer += 4;
                }
                Instruction::AdjustRelativeBase(_) => {
                    self.instruction_pointer += 2;
                }
                Instruction::Halt => {
                    self.instruction_pointer += 1;
                }
            }
            Ok(())
        } else {
            Err(CpuError::InvalidLastInstruction)
        }
    }

    /// Get the value at a memory address
    fn get_memory(&self, address: usize) -> isize {
        self.memory[address]
    }

    /// Convienient wrapper around `get_memory` that will get a value instead of a position if needed
    fn get_value(&self, address: usize) -> isize {
        self.get_memory(self.get_memory(address) as usize)
    }

    fn get_relative(&self, address: usize) -> isize {
        self.get_memory((self.relative_base + self.get_memory(address)) as usize)
    }

    fn set_memory(&mut self, address: usize, value: isize) {
        self.memory[address] = value;
    }

    pub fn set_exit_on_output(&mut self) {
        self.exit_on_output = true;
    }

    pub fn clear_exit_on_output(&mut self) {
        self.exit_on_output = false;
    }

    pub fn set_memory_size(&mut self, size: usize) {
        self.memory.resize(size, 0);
    }
    
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Add(isize, isize, isize),
    Mult(isize, isize, isize),
    In(isize),
    Out(isize),
    JumpIfTrue(isize, isize),
    JumpIfFalse(isize, isize),
    LessThan(isize, isize, isize),
    Equal(isize, isize, isize),
    AdjustRelativeBase(isize),
    Halt,
}

impl Instruction {
    fn parse(cpu: &CPU) -> CpuResult<Instruction> {
        let mem = cpu.get_memory(cpu.instruction_pointer);
        let flags = (mem / 100).to_string();
        let mut flags_iter = flags.chars().rev();
        let instr = match mem % 100 {
            1 => Instruction::Add(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                get_location!(flags_iter, cpu, cpu.instruction_pointer + 3),
            ),
            2 => Instruction::Mult(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                get_location!(flags_iter, cpu, cpu.instruction_pointer + 3),
            ),
            3 => Instruction::In(
                get_location!(flags_iter, cpu, cpu.instruction_pointer + 1)
            ),
            4 => Instruction::Out(address_or_value!(
                flags_iter,
                cpu,
                cpu.instruction_pointer + 1
            )),
            5 => Instruction::JumpIfTrue(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
            ),
            6 => Instruction::JumpIfFalse(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
            ),
            7 => Instruction::LessThan(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                get_location!(flags_iter, cpu, cpu.instruction_pointer + 3),
            ),
            8 => Instruction::Equal(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                get_location!(flags_iter, cpu, cpu.instruction_pointer + 3),
            ),
            9 => Instruction::AdjustRelativeBase (
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
            ),
            99 => Instruction::Halt,
            _ => {
                return Err(CpuError::InvalidOpcode(
                    cpu.get_memory(cpu.instruction_pointer),
                    cpu.instruction_pointer,
                ))
            }
        };

        Ok(instr)
    }
}

fn get_input() -> Result<isize, &'static str> {
    use std::io;

    debug_print!("Getting input!");

    let mut input = String::new();

    if let Err(_) = io::stdin().read_line(&mut input) {
        return Err("Invalid user input!");
    }
    let parse_input = input.trim().parse::<isize>();
    match parse_input {
        Err(_) => return Err("Error parsing user input!"),
        Ok(val) => return Ok(val),
    }
}

#[aoc(day9, part1)]
fn d9p1(input: &str) -> isize {
    let mut cpu = CPU::new(input);
    cpu.set_memory_size(2048);
    if let Err(error) = cpu.run(Some("1")) {
        println!("ERROR : {:?}", error);
    }
    *cpu.output.last().expect("Expected output from CPU")
}

#[aoc(day9, part2)]
fn d9p2(input: &str) -> isize {
    let mut cpu = CPU::new(input);
    cpu.set_memory_size(2048);
    if let Err(error) = cpu.run(Some("2")) {
        println!("ERROR : {:?}", error);
    }
    *cpu.output.last().expect("Expected output from CPU")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";

        let mut cpu = CPU::new(input);
        cpu.set_memory_size(1024);
        cpu.run(None);
        println!("{:?}", cpu.output);
        assert_eq!(cpu.output, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99])
    }

    #[test]
    fn test2() {
        let input = "1102,34915192,34915192,7,4,7,99,0";

        let mut cpu = CPU::new(input);
        cpu.run(None);
        println!("{:?}", cpu.output);
        assert_eq!(cpu.output, vec![1219070632396864])
    }

    #[test]
    fn test3() {
        let input = "104,1125899906842624,99";

        let mut cpu = CPU::new(input);
        cpu.run(None);
        println!("{:?}", cpu.output);
        assert_eq!(cpu.output, vec![1125899906842624])
    }
}