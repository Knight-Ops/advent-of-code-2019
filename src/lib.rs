extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

// #[macro_use]
// extern crate itertools;

// #[macro_use]
// extern crate lazy_static;

macro_rules! debug_print {
    ($fmt:expr) => {
        #[cfg(feature = "debugging")]
        {
            println!($fmt);
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[cfg(feature = "debugging")]
        {
            println!($fmt, $($arg)*);
        }
    };
}

mod intcode;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod day11;

pub mod day13;

aoc_lib! { year = 2019 }
