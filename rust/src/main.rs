#![allow(dead_code)]
use std::path::Path;
use std::{fs, io};

const BASE_URL: &str = "/home/garrick/Documents/personal/advent_of_code/src";
mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    // day_1::day_1();
    // day_2::day_2();
    // day_3::day_3();
    // day_4::day_4();
    // day_5::problem_1();
    // day_5::problem_2();
    // day_6::problem_1();
    // day_6::problem_2();
    // day_7::run();
    // day_8::run();
    // day_9::run();
    // day_10::run();
    // day_11::run();
    // day_12::run();
    // day_13::run();
    // day_14::run();
    day_15::run();
}

fn read_lines<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename)
}

fn read_input(day: &str) -> io::Result<String> {
    fs::read_to_string(format!("src/{day}/input.txt"))
}
