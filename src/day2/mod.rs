// #[aoc_generator(day2)]
// fn process_input(input: &str) -> Vec<usize> {
//     input
//         .split(",")
//         .map(|x| x.parse::<usize>().unwrap())
//         .collect()
// }

#[aoc(day2, part1)]
fn d2p1(input: &str) -> usize {
    let mut memory: Vec<usize> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut ip = 0;
    loop {
        let mut noun = memory[ip + 1];
        let mut verb = memory[ip + 2];
        let loc = memory[ip + 3];

        if ip == 0 {
            if noun == 0 && verb == 0 {
                memory[ip + 1] = 12;
                memory[ip + 2] = 2;
                noun = 12;
                verb = 2;
            }
        }

        match memory[ip] {
            1 => {
                memory[loc] = memory[noun] + memory[verb];
            }
            2 => {
                memory[loc] = memory[noun] * memory[verb];
            }
            99 => return memory[0],
            _ => unreachable!(),
        }

        ip += 4
    }
}

#[aoc(day2, part2)]
fn d2p2(input: &str) -> usize {
    for o in 0..100 {
        for i in 0..100 {
            let mut memory: Vec<usize> = input
                .trim()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let mut ip = 0;
            memory[1] = o;
            memory[2] = i;
            loop {
                let noun = memory[ip + 1];
                let verb = memory[ip + 2];
                let loc = memory[ip + 3];
                match memory[ip] {
                    1 => {
                        memory[loc] = memory[noun] + memory[verb];
                    }
                    2 => {
                        memory[loc] = memory[noun] * memory[verb];
                    }
                    99 => break,
                    _ => unreachable!(),
                }
                ip += 4
            }
            if memory[0] == 19690720 {
                return memory[1] * 100 + memory[2];
            }
        }
    }

    unreachable!();
}

#[aoc(day2, part1, rewrite)]
fn d2p1_rewrite(input: &str) -> isize {
    let mut cpu = CPU::new(input);
    cpu.set_memory(1, 12);
    cpu.set_memory(2, 2);
    if let Err(error) = cpu.run() {
        println!("ERROR : {:?}", error);
        0
    } else {
        cpu.get_memory(0)
    }
}

#[aoc(day2, part2, rewrite)]
fn d2p2_rewrite(input: &str) -> isize {
    use rayon::prelude::*;
    use std::sync::{
        atomic::{AtomicIsize, Ordering},
        Arc,
    };

    let value = Arc::new(AtomicIsize::new(0));

    (0 as isize..9999 as isize).into_par_iter().for_each(|x| {
        if value.load(Ordering::SeqCst) == 0 {
            let mut cpu = CPU::new(input);
            cpu.set_memory(1, x / 100);
            cpu.set_memory(2, x % 100);
            if let Err(_) = cpu.run() {
            } else {
                if cpu.get_memory(0) == 19690720 {
                    value.store(x, Ordering::SeqCst)
                }
            }
        }
    });

    value.load(Ordering::SeqCst) as isize
}

#[aoc(day2, part2, clone_cpu)]
fn d2p2_clone_cpu(input: &str) -> isize {
    use rayon::prelude::*;
    use std::sync::{
        atomic::{AtomicIsize, Ordering},
        Arc,
    };

    let value = Arc::new(AtomicIsize::new(0));
    let orig_cpu = CPU::new(input);

    (0 as isize..9999 as isize).into_par_iter().for_each(|x| {
        if value.load(Ordering::SeqCst) == 0 {
            let mut cpu = orig_cpu.clone();
            cpu.set_memory(1, x / 100);
            cpu.set_memory(2, x % 100);
            if let Err(_) = cpu.run() {
            } else {
                if cpu.get_memory(0) == 19690720 {
                    value.store(x, Ordering::SeqCst)
                }
            }
        }
    });

    value.load(Ordering::SeqCst) as isize
}

#[aoc(day2, part2, double_iter)]
fn d2p2_double_iter(input: &str) -> isize {
    use rayon::prelude::*;
    use std::sync::{
        atomic::{AtomicIsize, Ordering},
        Arc,
    };

    let value = Arc::new(AtomicIsize::new(0));
    let orig_cpu = CPU::new(input);

    (0 as isize..99 as isize).into_par_iter().for_each(|x| {
        (0 as isize..99 as isize).into_par_iter().for_each(|y| {
            if value.load(Ordering::SeqCst) == 0 {
                let mut cpu = orig_cpu.clone();
                cpu.set_memory(1, x);
                cpu.set_memory(2, y);
                if let Err(_) = cpu.run() {
                } else {
                    if cpu.get_memory(0) == 19690720 {
                        value.store(x * 100 + y, Ordering::SeqCst)
                    }
                }
            }
        })
    });

    value.load(Ordering::SeqCst) as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(d2p1(&process_input("1,9,10,3,2,3,11,0,99,30,40,50")), 3500);
    }
}

// ====================Day 5 Code=============================

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
}

#[derive(Debug, Clone)]
struct CPU {
    memory: Vec<isize>,
    instruction_pointer: usize,
    last_instruction: Option<Instruction>,
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
        }
    }

    pub fn run(&mut self) -> CpuResult<()> {
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
                    self.set_memory(location as usize, get_input()?);
                }
                Instruction::Out(value) => {
                    debug_print!("Out : {}", value);
                    self.last_instruction = Some(Instruction::Out(value));
                    println!("{}", value as usize);
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

fn get_input() -> CpuResult<isize> {
    use std::io;

    let mut input = String::new();

    if let Err(_) = io::stdin().read_line(&mut input) {
        return Err(CpuError::InvalidUserInput);
    }
    let parse_input = input.trim().parse::<isize>();
    match parse_input {
        Err(_) => return Err(CpuError::InvalidUserInput),
        Ok(val) => return Ok(val),
    }
}
