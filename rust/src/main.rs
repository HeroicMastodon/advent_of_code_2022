#![allow(dead_code)]
use std::path::Path;
use std::{fs, io};

const BASE_URL: &str = "/home/garrick/Documents/personal/advent_of_code/src";
mod day_01;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

fn main() {
    day_01::day_1();
    day_02::day_2();
    day_03::day_3();
    day_04::day_4();
    day_05::problem_1();
    day_05::problem_2();
    day_06::problem_1();
    day_06::problem_2();
    day_07::run();
    day_08::run();
    day_09::run();
    day_10::run();
    day_11::run();
    day_12::run();
    day_13::run();
    day_14::run();
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
