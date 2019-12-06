use rayon::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Debug, Copy, Clone)]
struct Password {
    start: usize,
    end: usize,
}

impl Password {
    fn new(start: usize, end: usize) -> Password {
        Password { start, end }
    }

    #[allow(dead_code)]
    fn six_digits(input: usize) -> bool {
        if input < 999999 && input > 100000 {
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn within_range(&self, input: usize) -> bool {
        if input < self.end && input > self.start {
            true
        } else {
            false
        }
    }

    fn two_adjacent_numbers(input: usize) -> bool {
        let string = input.to_string();
        for x in 1..string.len() {
            if &string[x..x + 1] == &string[x - 1..x] {
                return true;
            }
        }
        false
    }

    fn only_two_numbers(input: usize) -> bool {
        let mut ret_val = false;
        let string = input.to_string();
        string.chars().for_each(|x| {
            if string.matches(x).count() == 2 {
                ret_val = true;
                return;
            }
        });

        ret_val
    }

    /// Parallel implementation of searching for two numbers, ends up being slower or equal in time currently
    #[allow(dead_code)]
    fn par_only_two_numbers(input: usize) -> bool {
        let ret_val = Arc::new(AtomicBool::new(false));
        let string = input.to_string();
        string.par_chars().for_each(|x| {
            if ret_val.load(Ordering::SeqCst) == true {
                return;
            }
            if string.matches(x).count() == 2 {
                ret_val.store(true, Ordering::SeqCst);
            }
        });

        ret_val.load(Ordering::SeqCst)
    }

    fn increasing_numbers(input: usize) -> bool {
        let mut last_digit = 0;
        let mut always_increasing = true;
        input.to_string().chars().for_each(|x| {
            if x.to_digit(10).unwrap() < last_digit {
                always_increasing = false;
                return;
            }

            last_digit = x.to_digit(10).unwrap();
        });
        always_increasing
    }
}

#[aoc_generator(day4)]
fn process_input(input: &str) -> Password {
    let split: Vec<&str> = input.trim().split("-").collect();
    Password::new(
        split[0].parse::<usize>().unwrap(),
        split[1].parse::<usize>().unwrap(),
    )
}

#[aoc(day4, part1, adding)]
fn d4p1(input: &Password) -> usize {
    let mut passwords = 0;
    for password in input.start..input.end {
        if Password::increasing_numbers(password)
            && Password::two_adjacent_numbers(password)
        {
            passwords += 1;
        }
    }
    passwords
}

#[aoc(day4, part1, iter)]
fn d4p1_iter(input: &Password) -> usize {
    (input.start..input.end)
        .into_par_iter()
        .filter(|x| {
            Password::increasing_numbers(*x)
                && Password::two_adjacent_numbers(*x)
        })
        .collect::<Vec<usize>>()
        .len()
}

#[aoc(day4, part2, adding)]
fn d4p2(input: &Password) -> usize {
    let mut passwords = 0;
    for password in input.start..input.end {
        if Password::increasing_numbers(password)
            && Password::only_two_numbers(password)
        {
            passwords += 1;
        }
    }
    passwords
}

#[aoc(day4, part2, iter)]
fn d4p2_iter(input: &Password) -> usize {
    (input.start..input.end)
        .into_par_iter()
        .filter(|x| {
           Password::increasing_numbers(*x)
                && Password::only_two_numbers(*x)
        })
        .collect::<Vec<usize>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let number = 111111;
        assert_eq!(
            Password::six_digits(number)
                && Password::increasing_numbers(number)
                && Password::two_adjacent_numbers(number),
            true
        )
    }

    #[test]
    #[should_panic]
    fn test2() {
        let number = 223450;
        assert_eq!(
            Password::six_digits(number)
                && Password::increasing_numbers(number)
                && Password::two_adjacent_numbers(number),
            true
        )
    }

    #[test]
    #[should_panic]
    fn test3() {
        let number = 123789;
        assert_eq!(
            Password::six_digits(number)
                && Password::increasing_numbers(number)
                && Password::two_adjacent_numbers(number),
            true
        )
    }

    #[test]
    fn test4() {
        let number = 112233;
        assert_eq!(
            Password::six_digits(number)
                && Password::increasing_numbers(number)
                && Password::only_two_numbers(number),
            true
        )
    }

    #[test]
    #[should_panic]
    fn test5() {
        let number = 123444;
        assert_eq!(
            Password::six_digits(number)
                && Password::increasing_numbers(number)
                && Password::only_two_numbers(number),
            true
        )
    }

    #[test]
    fn test6() {
        let number = 111122;
        assert_eq!(
            Password::six_digits(number)
                && Password::increasing_numbers(number)
                && Password::only_two_numbers(number),
            true
        )
    }
}
