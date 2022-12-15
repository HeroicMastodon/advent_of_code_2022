#![allow(dead_code)]
use std::{fs, io};
use std::path::Path;

const BASE_URL: &str = "/home/garrick/Documents/personal/advent_of_code/src";
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;

fn main() {
day_14::run();
}

fn read_lines<P>(filename: P) -> io::Result<String>
    where P: AsRef<Path>, {
    fs::read_to_string(filename)
}

fn read_input(day: &str) -> io::Result<String> {
    fs::read_to_string(format!("src/{day}/input.txt"))
}
