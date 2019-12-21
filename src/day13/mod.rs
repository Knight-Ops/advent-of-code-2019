use crate::intcode::*;

use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fmt;

use num_enum::{TryFromPrimitive, IntoPrimitive};
use std::convert::TryFrom;
use std::convert::Into;

const SCREEN_SIZE : usize = 40;

#[derive(Debug, Copy, Clone)]
enum OutputType {
    XPosition,
    YPosition,
    TileID,
}

#[derive(Debug, Copy, Clone, IntoPrimitive)]
#[repr(isize)]
enum Direction {
    Left = -1,
    Neutral,
    Right,
}

#[derive(Debug, Clone)]
struct Arcade {
    brain: CPU,
    screen: HashMap<usize, Vec<Tile>>,
    joystick: Direction,
    score: isize,
}

impl Arcade {
    pub fn new(program: &str, memory_size: usize) -> Arcade {
        let mut arcade = Arcade {
            brain: CPU::new(program),
            screen: HashMap::new(),
            joystick: Direction::Neutral,
            score: 0,
        };
        arcade.brain.set_exit_on_output();
        arcade.brain.set_memory_size(memory_size);
        arcade
    }

    pub fn run(&mut self, input: Option<&str>) -> CpuResult<ExitReason> {
        let mut input = input;
        let mut input_string = String::from("");

        let mut output_type = OutputType::XPosition;
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut tile_id = Tile::Empty;
        loop {
            match self.brain.run(input) {
                Ok(ExitReason::Halt) => return Ok(ExitReason::Halt),
                Ok(ExitReason::InputRequired) => {
                    self.update_joystick_input();

                    input_string = format!("{}", isize::from(self.joystick));

                    input = Some(&input_string);
                    continue;
                }
                Ok(ExitReason::OutputGenerated) => {
                    match output_type {
                        OutputType::XPosition => {
                            x_pos = *self.brain.get_last_output().expect("No output for X Position");
                            output_type = OutputType::YPosition;
                        },
                        OutputType::YPosition => {
                            y_pos = *self.brain.get_last_output().expect("No output for Y Position");
                            output_type = OutputType::TileID;
                        }, 
                        OutputType::TileID => {
                            if x_pos == -1 && y_pos == 0 {
                                self.score = *self.brain.get_last_output().expect("No output for TileID");
                            } else {
                                tile_id = Tile::try_from(*self.brain.get_last_output().expect("No output for TileID")).expect("Invalid Tile ID");
                                self.tile_entry(x_pos as usize, y_pos as usize, tile_id);
                            }
                            output_type = OutputType::XPosition;
                        },
                    }
                },
                _ => {
                    println!("Error");
                }
            }
            input = None;
        }
    }

    fn tile_entry(&mut self, x: usize, y: usize, tile_id: Tile) {

        self.screen.entry(y).or_insert(vec![Tile::Empty; SCREEN_SIZE])[x] = tile_id;

    }

    pub fn count_blocks(&self) -> usize {
        let mut total_blocks = 0;
        self.screen.iter().for_each(|row| {
            total_blocks += (row.1).iter().filter(|item| **item == Tile::Block).count();
        });
        total_blocks
    }

    fn update_joystick_input(&mut self) {
        let ball = self.find_ball();
        let paddle = self.find_paddle();

        if ball.0 < paddle.0 {
            self.joystick = Direction::Left;
        } else if ball.0 > paddle.0 {
            self.joystick = Direction::Right;
        } else {
            self.joystick = Direction::Neutral;
        }
    }

    fn find_ball(&self) -> (usize, usize) {
        let mut x_coord = 0;
        let mut y_coord = 0;
        self.screen.iter().filter(|entry| (entry.1).contains(&Tile::Ball)).for_each(|entry| {
            y_coord = *entry.0;
            x_coord = entry.1.iter().position(|&x| x == Tile::Ball).expect("Can't find ball in Ball row");
        });
        (x_coord, y_coord)
    }

    fn find_paddle(&self) -> (usize, usize) {
        let mut x_coord = 0;
        let mut y_coord = 0;
        self.screen.iter().filter(|entry| (entry.1).contains(&Tile::HorizontalPaddle)).for_each(|entry| {
            y_coord = *entry.0;
            x_coord = entry.1.iter().position(|&x| x == Tile::HorizontalPaddle).expect("Can't find horizontalPaddle in horizontalPaddle row");
        });
        (x_coord, y_coord)
    }

    pub fn get_score(&self) -> isize {
        self.score
    }

}

impl fmt::Display for Arcade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (0..SCREEN_SIZE).for_each(|row_num| {
            match self.screen.get(&row_num) {
                Some(vector) => {
                    for tile in vector {
                        match *tile {
                            Tile::Empty => {
                                write!(f, " ");
                            },
                            Tile::Wall => {
                                write!(f, "|");
                            },
                            Tile::Block => {
                                write!(f, "■");
                            },
                            Tile::HorizontalPaddle => {
                                write!(f, "_");
                            }
                            Tile::Ball => {
                                write!(f, "⚽");
                            }
                        }
                    }
                    write!(f, "\n");
                },
                None => {
                    write!(f, "");
                },
            }
        });

        write!(f, "Score : {}", self.score)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(isize)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}


#[aoc(day13, part1)]
fn d13p1(input: &str) -> usize {
    let mut arcade = Arcade::new(input, 4096);
    if let Err(error) = arcade.run(None) {
        println!("ERROR : {:?}", error);
    }
    arcade.count_blocks()
}

#[aoc(day13, part2)]
fn d13p2(input: &str) -> usize {
    let mut arcade = Arcade::new(input, 4096);
    arcade.brain.set_memory(0, 2);
    if let Err(error) = arcade.run(None) {
        println!("ERROR : {:?}", error);
    }
    arcade.get_score() as usize
}
