#[aoc_generator(day2)]
fn process_input(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn d2p1(input: &Vec<usize>) -> usize {
    let mut memory: Vec<usize> = input.to_vec();
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
fn d2p2(input: &Vec<usize>) -> usize {
    for o in 0..100 {
        for i in 0..100 {
            let mut memory: Vec<usize> = input.to_vec();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(d2p1(&process_input("1,9,10,3,2,3,11,0,99,30,40,50")), 3500);
    }
}
