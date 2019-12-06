use rayon::prelude::*;

#[aoc_generator(day1)]
fn process_input(input: &str) -> Vec<isize> {
    input.lines().map(|x| x.parse::<isize>().unwrap()).collect()
}

#[aoc(day1, part1, pure_iter)]
fn d1p1(input: &Vec<isize>) -> isize {
    input.iter().map(|x| (x / 3) - 2).sum()
}

#[aoc(day1, part1, rayon_iter)]
fn d1p1_rayon(input: &Vec<isize>) -> isize {
    input.par_iter().map(|x| (x / 3) - 2).sum()
}

#[aoc(day1, part2, pure_iter_2)]
fn d1p2(input: &Vec<isize>) -> isize {
    // let mut modules = vec![];

    input
        .iter()
        .map(|x| {
            let mut fuel = (x / 3) - 2;
            let mut total = fuel;

            loop {
                fuel = (fuel / 3) - 2;

                if fuel <= 0 {
                    break;
                } else {
                    total += fuel;
                }
            }
            total
        })
        .sum()
}

#[aoc(day1, part2, rayon_iter_2)]
fn d1p2_rayon(input: &Vec<isize>) -> isize {
    input
        .par_iter()
        .map(|x| {
            let mut fuel = (x / 3) - 2;
            let mut total = fuel;

            loop {
                fuel = (fuel / 3) - 2;

                if fuel <= 0 {
                    break;
                } else {
                    total += fuel;
                }
            }
            total
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(d1p1(&vec!(12)), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(d1p1(&vec!(14)), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(d1p1(&vec!(1969)), 654);
    }

    #[test]
    fn test4() {
        assert_eq!(d1p1(&vec!(100756)), 33583);
    }

    #[test]
    fn test5() {
        assert_eq!(d1p1_rayon(&vec!(12)), 2);
    }

    #[test]
    fn test6() {
        assert_eq!(d1p1_rayon(&vec!(14)), 2);
    }

    #[test]
    fn test7() {
        assert_eq!(d1p1_rayon(&vec!(1969)), 654);
    }

    #[test]
    fn test8() {
        assert_eq!(d1p1_rayon(&vec!(100756)), 33583);
    }

    #[test]
    fn test9() {
        assert_eq!(d1p2(&vec!(14)), 2);
    }

    #[test]
    fn test10() {
        assert_eq!(d1p2(&vec!(1969)), 966);
    }

    #[test]
    fn test11() {
        assert_eq!(d1p2(&vec!(100756)), 50346);
    }
}
