use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{hash_map::Entry, HashMap, HashSet};

macro_rules! address_or_value {
    ($iter_name:ident, $cpu:ident, $expression:expr) => {
        if $iter_name.next() == Some('1') {
            $cpu.get_memory($expression)
        } else {
            $cpu.get_value($expression)
        }
    };
}

type CpuResult<T> = std::result::Result<T, CpuError>;

#[derive(Debug, Clone, Copy)]
enum CpuError {
    InvalidOpcode(isize, usize),
    InvalidLastInstruction,
    InvalidUserInput,
    InputRequired,
    OutputGenerated,
}

#[derive(Debug, Clone)]
struct CPU {
    memory: Vec<isize>,
    instruction_pointer: usize,
    last_instruction: Option<Instruction>,
    output: Vec<usize>,
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
            last_instruction: None,
            output: vec![],
        }
    }

    pub fn run(&mut self, input: Option<&str>) -> CpuResult<()> {
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
                                None => return Err(CpuError::InputRequired),
                            }
                        } else {
                            return Err(CpuError::InputRequired);
                        },
                    );
                }
                Instruction::Out(value) => {
                    debug_print!("Out : {}", value);
                    self.last_instruction = Some(Instruction::Out(value));
                    self.output.push(value as usize);
                    self.increment_ip()?;
                    return Err(CpuError::OutputGenerated);
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
                    debug_print!("EQ : {} < {} @ {}", left, right, location);
                    self.last_instruction = Some(Instruction::Equal(left, right, location));
                    if left == right {
                        self.set_memory(location as usize, 1);
                    } else {
                        self.set_memory(location as usize, 0);
                    }
                }
                Instruction::Halt => {
                    debug_print!("Halt");
                    self.last_instruction = Some(Instruction::Halt);
                    return Ok(());
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

    fn set_memory(&mut self, address: usize, value: isize) {
        self.memory[address] = value;
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
                cpu.get_memory(cpu.instruction_pointer + 3),
            ),
            2 => Instruction::Mult(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                cpu.get_memory(cpu.instruction_pointer + 3),
            ),
            3 => Instruction::In(cpu.get_memory(cpu.instruction_pointer + 1)),
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
                cpu.get_memory(cpu.instruction_pointer + 3),
            ),
            8 => Instruction::Equal(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                cpu.get_memory(cpu.instruction_pointer + 3),
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

#[aoc(day7, part1)]
fn d7p1(input: &str) -> usize {
    let orig_cpu = CPU::new(input);

    let mut output = 0;
    let mut max = 0;
    (0..5).permutations(5).for_each(|permutation| {
        debug_print!("Permutation : {:?}", permutation);
        permutation.iter().for_each(|entry| {
            let mut amp = orig_cpu.clone();
            let mut formatting = format!("{}\n{}", entry, output);
            let mut vm_input = Some(formatting.as_str());
            loop {
                match amp.run(vm_input) {
                    CpuResult::Ok(_) => {
                        output = *amp.output.last().expect("No output provided by VM");
                        debug_print!("Completed : {}", output);
                        break;
                    }
                    CpuResult::Err(err) => match err {
                        CpuError::InputRequired => {
                            let val = get_input().expect("Error while getting user input");
                            formatting = format!("{}", val);
                            vm_input = Some(formatting.as_str());
                        }
                        CpuError::OutputGenerated => {}
                        _ => {
                            panic!(
                                "Error while running through : {:?} - {:?}",
                                permutation, err
                            );
                        }
                    },
                }
            }
        });

        if output > max {
            max = output;
        }

        output = 0;
    });

    max
}

#[aoc(day7, part2)]
fn d7p2(input: &str) -> usize {
    let orig_cpu = CPU::new(input);
    let orig_amps: Vec<CPU> = (0..5).map(|_| orig_cpu.clone()).collect();
    let mut max = 0;
    (5..10).permutations(5).for_each(|permutation| {
        debug_print!("Permutation : {:?}", permutation);
        let mut output = 0;
        let mut list_iter = permutation.iter();
        let mut amps = orig_amps.clone();
        let mut active_amp = 0;
        let mut first_pass = true;
        let mut formatting = format!(
            "{}\n{}",
            list_iter
                .next()
                .expect("Permutation does not contain a single element"),
            output
        );
        let mut vm_input = Some(formatting.as_str());
        loop {
            match amps[active_amp].run(vm_input) {
                CpuResult::Ok(_) => {
                    output = *amps[active_amp]
                        .output
                        .last()
                        .expect("No output provided by VM");
                    debug_print!("Completed : {}", output);
                    if active_amp == orig_amps.len() - 1 {
                        break;
                    } else {
                        active_amp = (active_amp + 1) % orig_amps.len();
                    }
                }
                CpuResult::Err(err) => match err {
                    CpuError::InputRequired => {
                        let val = get_input().expect("Error while getting user input");
                        formatting = format!("{}", val);
                        vm_input = Some(formatting.as_str());
                    }
                    CpuError::OutputGenerated => {
                        output = *amps[active_amp].output.last().unwrap();
                        active_amp = (active_amp + 1) % orig_amps.len();
                        if active_amp == 0 && first_pass {
                            first_pass = false;
                        }

                        if first_pass {
                            formatting = format!(
                                "{}\n{}",
                                list_iter
                                    .next()
                                    .expect("Permutation does not have enough elements"),
                                output
                            );
                            vm_input = Some(formatting.as_str());
                        } else {
                            formatting = format!("{}", output);
                            vm_input = Some(formatting.as_str());
                        }
                    }
                    _ => {
                        panic!(
                            "Error while running through : {:?} - {:?}",
                            permutation, err
                        );
                    }
                },
            }
        }

        if output > max {
            max = output;
        }
    });

    max
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            d7p1(&"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            d7p1(&"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
            54321
        );
    }

    #[test]
    fn test3() {
        assert_eq!(d7p1(&"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
    }

    #[test]
    fn test4() {
        assert_eq!(d7p2(&"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139629729);
    }

    #[test]
    fn test5() {
        assert_eq!(d7p2(&"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
    }
}
