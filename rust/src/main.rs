use std::fs::File;
use std::{fs, io};
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;
use day_7::run;

const BASE_URL: &str = "/home/garrick/Documents/personal/advent_of_code/src";
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

fn main() {
run();
}

fn read_lines<P>(filename: P) -> std::io::Result<String>
    where P: AsRef<Path>, {
    fs::read_to_string(filename)
}

fn read_input(day: &str) -> io::Result<String> {
    fs::read_to_string(format!("src/{day}/input.txt"))
}
