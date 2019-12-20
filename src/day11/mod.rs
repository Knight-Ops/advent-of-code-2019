use crate::intcode::*;

use itertools::Itertools;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use rayon::prelude::*;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::convert::From;
use std::convert::Into;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(isize)]
enum Color {
    Black,
    White,
}

impl Color {
    fn to_string(&self) -> String {
        format!("{}\n", *self as u8)
    }
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(isize)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(isize)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum OutputType {
    Paint,
    Move,
}

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Coordinates {
    pub fn new() -> Coordinates {
        Coordinates { x: 0, y: 0 }
    }
}

impl From<(isize, isize)> for Coordinates {
    fn from(tuple: (isize, isize)) -> Self {
        Coordinates {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Into<(isize, isize)> for Coordinates {
    fn into(self) -> (isize, isize) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Robot {
    brain: CPU,
    direction: Direction,
    location: Coordinates,
    camera: HashMap<(isize, isize), Color>,
}

impl Robot {
    pub fn new(program: &str, memory_size: usize) -> Robot {
        let mut robot = Robot {
            brain: CPU::new(program),
            direction: Direction::Up,
            location: Coordinates::new(),
            camera: HashMap::new(),
        };
        robot.brain.set_exit_on_output();
        robot.brain.set_memory_size(memory_size);
        robot
    }

    pub fn run(&mut self, input: Option<&str>) -> CpuResult<ExitReason> {
        let mut input = input;
        let mut input_string = String::from("");
        let mut output_type = OutputType::Paint;
        loop {
            match self.brain.run(input) {
                Ok(ExitReason::Halt) => return Ok(ExitReason::Halt),
                Ok(ExitReason::InputRequired) => {
                    let color = self.read_camera();
                    input_string = color.to_string();
                    input = Some(&input_string);
                    debug_print!("Tile is : {:?}\nInput is : {:?}", color, input);
                    continue;
                }
                Ok(ExitReason::OutputGenerated) => match output_type {
                    OutputType::Paint => {
                        let color = Color::try_from(
                            *self.brain.get_last_output().expect("No output in CPU"),
                        );
                        debug_print!("We need to paint : {:?}", color);

                        if let Ok(color) = color {
                            self.camera.insert(self.location.into(), color);
                        } else {
                            return Err(CpuError::InvalidOutputGenerated);
                        }
                        output_type = OutputType::Move;
                    }
                    OutputType::Move => {
                        let direction = TurnDirection::try_from(
                            *self.brain.get_last_output().expect("No output in CPU"),
                        );

                        if let Ok(direction) = direction {
                            self.move_robot(direction);
                        } else {
                            return Err(CpuError::InvalidOutputGenerated);
                        }

                        output_type = OutputType::Paint;
                    }
                },
                _ => {
                    println!("Error");
                }
            }
            input = None;
        }
    }

    fn read_camera(&mut self) -> Color {
        debug_print!("Reading Paint");
        *self
            .camera
            .entry(self.location.into())
            .or_insert(Color::Black)
    }

    fn move_robot(&mut self, movement_direction: TurnDirection) {
        match self.direction {
            Direction::Up => match movement_direction {
                TurnDirection::Left => {
                    self.location.x -= 1;
                    self.direction = Direction::Left;
                }
                TurnDirection::Right => {
                    self.location.x += 1;
                    self.direction = Direction::Right;
                }
            },
            Direction::Left => match movement_direction {
                TurnDirection::Left => {
                    self.location.y -= 1;
                    self.direction = Direction::Down;
                }
                TurnDirection::Right => {
                    self.location.y += 1;
                    self.direction = Direction::Up;
                }
            },
            Direction::Down => match movement_direction {
                TurnDirection::Left => {
                    self.location.x += 1;
                    self.direction = Direction::Right;
                }
                TurnDirection::Right => {
                    self.location.x -= 1;
                    self.direction = Direction::Left;
                }
            },
            Direction::Right => match movement_direction {
                TurnDirection::Left => {
                    self.location.y += 1;
                    self.direction = Direction::Up;
                }
                TurnDirection::Right => {
                    self.location.y -= 1;
                    self.direction = Direction::Down;
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
struct RenderableImage {
    image: HashMap<usize, Vec<Pixel>>,
    height: usize,
    width: usize,
}

impl fmt::Display for RenderableImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (0..self.height).for_each(|row_num| {
            self.image
                .get(&row_num)
                .unwrap()
                .iter()
                .for_each(|pixel| match pixel {
                    Pixel::Black => write!(f, "■").expect("Pixel::Black"),
                    Pixel::White => write!(f, "□").expect("Pixel::White"),
                    Pixel::Transparent => write!(f, " ").expect("Pixel::Transparent"),
                });
            write!(f, "\n").expect("here");
        });
        write!(f, "")
    }
}

impl From<HashMap<(isize, isize), Color>> for RenderableImage {
    fn from(input: HashMap<(isize, isize), Color>) -> Self {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        input.iter().for_each(|x| {
            let coords = x.0;
            let color = x.1;
            if coords.0 > max_x {
                max_x = coords.0;
            }
            if coords.0 < min_x {
                min_x = coords.0;
            }
            if coords.1 > max_y {
                max_y = coords.1;
            }
            if coords.1 < min_y {
                min_y = coords.1;
            }
        });

        let width = ((max_x - min_x).abs() + 1) as usize;
        let height = ((max_y - min_y).abs() + 1) as usize;

        let mut image: HashMap<usize, Vec<Pixel>> = HashMap::new();

        input.iter().for_each(|x| {
            let vector = image
                .entry(((x.0).1).abs() as usize)
                .or_insert(vec![Pixel::Black; width]);
            vector[((x.0).0) as usize] = Pixel::from(*x.1)
        });

        RenderableImage {
            image,
            height,
            width,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Pixel {
    Transparent,
    White,
    Black,
}

impl From<Color> for Pixel {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => Pixel::Black,
            Color::White => Pixel::White,
        }
    }
}

#[aoc(day11, part1)]
fn d11p1(input: &str) -> usize {
    let mut robot = Robot::new(input, 2048);
    if let Err(error) = robot.run(None) {
        println!("ERROR : {:?}", error);
    }
    robot.camera.len()
}

#[aoc(day11, part2)]
fn d11p2(input: &str) -> usize {
    let mut robot = Robot::new(input, 2048);
    robot.camera.insert((0, 0), Color::White);
    if let Err(error) = robot.run(None) {
        println!("ERROR : {:?}", error);
    }
    let image = RenderableImage::from(robot.camera);
    println!("{}", image);
    0
}
