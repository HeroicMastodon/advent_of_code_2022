use crate::{read_input, read_lines};
use std::collections::HashSet;

fn find_unique(input: String, size: usize) -> u32 {
    let mut idx = 0;
    let mut set: HashSet<char> = HashSet::new();

    while set.len() < size {
        set = input.chars().skip(idx).take(size).collect();
        idx += 1;
    }

    (idx + size - 1) as u32
}

pub fn problem_1() {
    let file = read_input("day_6").unwrap();
    let result = find_unique(file, 4);
    println!("{result}");
}

pub fn problem_2() {
    let file = read_input("day_6").unwrap();
    let result = find_unique(file, 14);
    println!("{result}");
}

#[test]
fn test() {
    let mut input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let mut expected = 7;
    let mut actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    let mut input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let mut expected = 5;
    let mut actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    let mut input = "nppdvjthqldpwncqszvftbrmjlhg";
    let mut expected = 6;
    let mut actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    let mut input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let mut expected = 10;
    let mut actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    let mut input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let mut expected = 11;
    let mut actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    // part 2
    let mut input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let mut expected = 19;
    let mut actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);

    let mut input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let mut expected = 23;
    let mut actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);

    let mut input = "nppdvjthqldpwncqszvftbrmjlhg";
    let mut expected = 23;
    let mut actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);

    let mut input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let mut expected = 29;
    let mut actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);

    let mut input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let mut expected = 26;
    let mut actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);
}
