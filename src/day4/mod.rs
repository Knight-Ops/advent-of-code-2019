#[derive(Debug, Copy, Clone)]
struct Password {
    start: usize,
    end: usize,
}

impl Password {
    fn new(start: usize, end: usize) -> Password {
        Password { start, end }
    }

    fn six_digits(input: usize) -> bool {
        if input < 999999 && input > 100000 {
            true
        } else {
            false
        }
    }

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

    fn increasing_numbers(input: usize) -> bool {
        let mut last_digit = 0;
        let mut always_increasing = true;
        input.to_string().chars().for_each(|x| {
            if x.to_digit(10).unwrap() < last_digit {
                always_increasing = false;
            }

            last_digit = x.to_digit(10).unwrap();
        });
        always_increasing
    }
}

#[aoc_generator(day4)]
fn process_input(input: &str) -> Password {
    let split: Vec<&str> = input.split("-").collect();
    Password::new(
        split[0].parse::<usize>().unwrap(),
        split[1].parse::<usize>().unwrap(),
    )
}

#[aoc(day4, part1)]
fn d3p1(input: &Password) -> usize {
    let mut passwords = 0;
    for password in input.start..input.end {
        if Password::six_digits(password)
            && Password::two_adjacent_numbers(password)
            && Password::increasing_numbers(password)
        {
            passwords += 1;
        }
    }
    passwords
}

#[aoc(day4, part2)]
fn d3p2(input: &Password) -> usize {
    0
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
    fn test4() {}

    #[test]
    fn test5() {}
}
