use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct WireHarness {
    first_wire: Vec<WireDirection>,
    second_wire: Vec<WireDirection>,
}

impl WireHarness {
    fn new(first_wire: &str, second_wire: &str) -> WireHarness {
        WireHarness {
            first_wire: WireHarness::parse_wire_str(first_wire),
            second_wire: WireHarness::parse_wire_str(second_wire),
        }
    }

    fn parse_wire_str(wire_str: &str) -> Vec<WireDirection> {
        wire_str
            .split(",")
            .map(|x| {
                let direction = &x[0..1];
                let distance = x[1..].parse::<usize>().unwrap();

                WireDirection::new(direction, distance)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
enum WireDirection {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl WireDirection {
    fn new(wire_direction: &str, wire_distance: usize) -> WireDirection {
        match wire_direction {
            "U" => WireDirection::Up(wire_distance),
            "D" => WireDirection::Down(wire_distance),
            "L" => WireDirection::Left(wire_distance),
            "R" => WireDirection::Right(wire_distance),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day3)]
fn process_input(input: &str) -> WireHarness {
    let mut wires = input.lines();
    WireHarness::new(wires.next().unwrap(), wires.next().unwrap())
}

#[aoc(day3, part1)]
fn d3p1(input: &WireHarness) -> usize {
    let mut first_wire_map: HashSet<(isize, isize)> = HashSet::new();
    let mut second_wire_map: HashSet<(isize, isize)> = HashSet::new();
    let mut x_cord = 0;
    let mut y_cord = 0;
    input
        .first_wire
        .iter()
        .map(|x| match x {
            WireDirection::Up(val) => {
                for _ in 0..*val {
                    y_cord += 1;
                    first_wire_map.insert((x_cord, y_cord));
                }
            }
            WireDirection::Down(val) => {
                for _ in 0..*val {
                    y_cord -= 1;
                    first_wire_map.insert((x_cord, y_cord));
                }
            }
            WireDirection::Left(val) => {
                for _ in 0..*val {
                    x_cord -= 1;
                    first_wire_map.insert((x_cord, y_cord));
                }
            }
            WireDirection::Right(val) => {
                for _ in 0..*val {
                    x_cord += 1;
                    first_wire_map.insert((x_cord, y_cord));
                }
            }
        })
        .for_each(|_| {});

    x_cord = 0;
    y_cord = 0;
    input
        .second_wire
        .iter()
        .map(|x| match x {
            WireDirection::Up(val) => {
                for _ in 0..*val {
                    y_cord += 1;
                    second_wire_map.insert((x_cord, y_cord));
                }
            }
            WireDirection::Down(val) => {
                for _ in 0..*val {
                    y_cord -= 1;
                    second_wire_map.insert((x_cord, y_cord));
                }
            }
            WireDirection::Left(val) => {
                for _ in 0..*val {
                    x_cord -= 1;
                    second_wire_map.insert((x_cord, y_cord));
                }
            }
            WireDirection::Right(val) => {
                for _ in 0..*val {
                    x_cord += 1;
                    second_wire_map.insert((x_cord, y_cord));
                }
            }
        })
        .for_each(|_| {});

    let mut min_distance = 0xFFFFFFFF;
    first_wire_map.intersection(&second_wire_map).for_each(|x| {
        if x.0.abs() + x.1.abs() < min_distance {
            println!("({}, {}) = {}", x.0, x.1, x.0.abs() + x.1.abs());
            min_distance = x.0.abs() + x.1.abs();
        }
    });
    min_distance as usize
}

#[aoc(day3, part2)]
fn d3p2(input: &WireHarness) -> usize {
    let mut first_wire_map: HashMap<(isize, isize), isize> = HashMap::new();
    let mut second_wire_map: HashMap<(isize, isize), isize> = HashMap::new();
    // let mut intersections: Vec<(isize, isize)> = vec![];
    let mut x_cord = 0;
    let mut y_cord = 0;
    let mut total_traveled = 0;
    input
        .first_wire
        .iter()
        .map(|x| match x {
            WireDirection::Up(val) => {
                for _ in 0..*val {
                    y_cord += 1;
                    total_traveled += 1;
                    match first_wire_map.entry((x_cord, y_cord)) {
                        Entry::Occupied(o) => {}
                        Entry::Vacant(o) => {
                            o.insert(total_traveled);
                        }
                    }
                }
            }
            WireDirection::Down(val) => {
                for _ in 0..*val {
                    y_cord -= 1;
                    total_traveled += 1;
                    match first_wire_map.entry((x_cord, y_cord)) {
                        Entry::Occupied(o) => {}
                        Entry::Vacant(o) => {
                            o.insert(total_traveled);
                        }
                    }
                }
            }
            WireDirection::Left(val) => {
                for _ in 0..*val {
                    x_cord -= 1;
                    total_traveled += 1;
                    match first_wire_map.entry((x_cord, y_cord)) {
                        Entry::Occupied(o) => {}
                        Entry::Vacant(o) => {
                            o.insert(total_traveled);
                        }
                    }
                }
            }
            WireDirection::Right(val) => {
                for _ in 0..*val {
                    x_cord += 1;
                    total_traveled += 1;
                    match first_wire_map.entry((x_cord, y_cord)) {
                        Entry::Occupied(o) => {}
                        Entry::Vacant(o) => {
                            o.insert(total_traveled);
                        }
                    }
                }
            }
        })
        .for_each(|_| {});

    x_cord = 0;
    y_cord = 0;
    total_traveled = 0;
    input
        .second_wire
        .iter()
        .map(|x| match x {
            WireDirection::Up(val) => {
                for _ in 0..*val {
                    y_cord += 1;
                    total_traveled += 1;
                    match second_wire_map.entry((x_cord, y_cord)) {
                        Entry::Occupied(o) => {}
                        Entry::Vacant(o) => {
                            o.insert(total_traveled);
                        }
                    }
                }
            }
            WireDirection::Down(val) => {
                for _ in 0..*val {
                    y_cord -= 1;
                    total_traveled += 1;
                    match second_wire_map.entry((x_cord, y_cord)) {
                        Entry::Occupied(o) => {}
                        Entry::Vacant(o) => {
                            o.insert(total_traveled);
                        }
                    }
                }
            }
            WireDirection::Left(val) => {
                for _ in 0..*val {
                    x_cord -= 1;
                    total_traveled += 1;
                    match second_wire_map.entry((x_cord, y_cord)) {
                        Entry::Occupied(o) => {}
                        Entry::Vacant(o) => {
                            o.insert(total_traveled);
                        }
                    }
                }
            }
            WireDirection::Right(val) => {
                for _ in 0..*val {
                    x_cord += 1;
                    total_traveled += 1;
                    match second_wire_map.entry((x_cord, y_cord)) {
                        Entry::Occupied(o) => {}
                        Entry::Vacant(o) => {
                            o.insert(total_traveled);
                        }
                    }
                }
            }
        })
        .for_each(|_| {});

    let mut min_distance = 0xFFFFFFFF;
    first_wire_map
        .drain()
        .for_each(|(key, value)| match second_wire_map.get(&(key.0, key.1)) {
            Some(second_value) => {
                if value + second_value < min_distance {
                    min_distance = value + second_value;
                }
            }
            None => {}
        });
    min_distance as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(d3p1(&process_input("R8,U5,L5,D3\nU7,R6,D4,L4")), 6)
    }

    #[test]
    fn test2() {
        assert_eq!(
            d3p1(&process_input(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            159
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            d3p1(&process_input(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            135
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            d3p2(&process_input(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        )
    }

    #[test]
    fn test5() {
        assert_eq!(
            d3p2(&process_input(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        )
    }
}
