use std::str::FromStr;
use crate::read_lines;

#[derive(Debug)]
struct ElfRange {
    pub lower: u32,
    pub upper: u32,
}

impl ElfRange {
    pub fn contains(&self, other: &ElfRange) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    pub fn overlaps(&self, other: &ElfRange) -> bool {
        // let self_range = ( self.lower..=self.upper ).collect::<HashSet<u32>>();
        // let other_range = ( other.lower..=other.upper ).collect::<HashSet<u32>>();
        
        // self_range.contains(&other.lower) ||
        //     self_range.contains(&other.upper) ||
        //     other_range.contains(&self.lower) ||
        //     other_range.contains(&self.upper)
        
        // self_range.intersection(&other_range).any(|_| true)
        
        self.lower <= other.lower && self.upper >= other.lower ||
            other.lower <= self.lower && other.upper >= self.lower
    }

    pub fn from_string(string: impl Into<String>) -> ElfRange {
        let str = string.into();
        let split = str.split("-").collect::<Vec<&str>>();
        ElfRange {
            lower: u32::from_str(split[0]).unwrap(),
            upper: u32::from_str(split[1]).unwrap(),
        }
    }
}

pub fn day_4() {
    let file = read_lines("src/day_4/input.txt").unwrap();

    let lines = file.lines();
    let result = lines.fold(0, |acc, line| {
        let split = line.split(",").collect::<Vec<&str>>();
        let (first, second) = (ElfRange::from_string(split[0]), ElfRange::from_string(split[1]));

        if first.overlaps(&second) {
            acc + 1
        } else {
            acc + 0
        }
    });

    println!("{result:#?}")
}