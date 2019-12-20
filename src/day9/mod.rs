use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::intcode::*;

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
    *cpu.get_output().last().expect("Expected output from CPU")
}

#[aoc(day9, part2)]
fn d9p2(input: &str) -> isize {
    let mut cpu = CPU::new(input);
    cpu.set_memory_size(2048);
    if let Err(error) = cpu.run(Some("2")) {
        println!("ERROR : {:?}", error);
    }
    *cpu.get_output().last().expect("Expected output from CPU")
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
        println!("{:?}", cpu.get_output());
        assert_eq!(
            cpu.get_output(),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        )
    }

    #[test]
    fn test2() {
        let input = "1102,34915192,34915192,7,4,7,99,0";

        let mut cpu = CPU::new(input);
        cpu.run(None);
        println!("{:?}", cpu.get_output());
        assert_eq!(cpu.get_output(), vec![1219070632396864])
    }

    #[test]
    fn test3() {
        let input = "104,1125899906842624,99";

        let mut cpu = CPU::new(input);
        cpu.run(None);
        println!("{:?}", cpu.get_output());
        assert_eq!(cpu.get_output(), vec![1125899906842624])
    }
}
