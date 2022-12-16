use crate::{read_input };
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
    let file = read_input("day_06").unwrap();
    let result = find_unique(file, 4);
    println!("{result}");
}

pub fn problem_2() {
    let file = read_input("day_06").unwrap();
    let result = find_unique(file, 14);
    println!("{result}");
}

#[test]
fn test() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let expected = 7;
    let actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let expected = 5;
    let actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    let expected = 6;
    let actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let expected = 10;
    let actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let expected = 11;
    let actual = find_unique(input.to_string(), 4);
    assert_eq!(actual, expected);

    // part 2
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let expected = 19;
    let actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);

    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let expected = 23;
    let actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);

    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    let expected = 23;
    let actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);

    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let expected = 29;
    let actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);

    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let expected = 26;
    let actual = find_unique(input.to_string(), 14);
    assert_eq!(actual, expected);
}
