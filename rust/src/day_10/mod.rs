use crate::day_10::Instruction::NoOp;
use std::str::FromStr;

pub fn run() {
    let input = include_str!("input.txt");
    println!("problem 1: {:#?}", problem_1(input.to_string()));
    println!("problem 2:\n{}", problem_2(input.to_string()));
}

fn problem_1(input: String) -> i32 {
    let instructions = parse_input(input);
    instructions
        .iter()
        .fold(
            (vec![], 1),
            |(mut cycles, prev): (Vec<i32>, i32), instruction| match instruction {
                NoOp => {
                    cycles.push(prev);
                    (cycles, prev)
                }
                Instruction::Add(count) => {
                    cycles.push(prev);
                    cycles.push(prev);
                    (cycles, count + prev)
                }
            },
        )
        .0
        .iter()
        .enumerate()
        .map(|(idx, val)| (idx + 1, val))
        .fold(0, |count, (idx, value)| {
            let offset_idx = idx as i32 - 20;
            if idx == 20 || offset_idx % 40 == 0 {
                let result = count + (idx as i32) * value;
                println!("{:#?}", &result);
                result
            } else {
                count
            }
        })
}

fn problem_2(input: String) -> String {
    let cycles = parse_input(input)
        .iter()
        .fold(
            (vec![], 1),
            |(mut cycles, prev): (Vec<i32>, i32), instruction| match instruction {
                NoOp => {
                    cycles.push(prev);
                    (cycles, prev)
                }
                Instruction::Add(count) => {
                    cycles.push(prev);
                    cycles.push(prev);
                    (cycles, count + prev)
                }
            },
        )
        .0;

    let mut screen = vec![vec![" "; 40]; 6];

    for (cycle, value) in cycles.iter().enumerate() {
        let col = cycle % 40;
        if col as i32 <= value + 1 && col as i32 >= value - 1 {
            let row = cycle / 40;
            screen[row][col] = "#";
        }
    }

    screen
        .iter()
        .map(|line| line.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

enum Instruction {
    NoOp,
    Add(i32),
}

fn parse_input(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(
            |line| match line.split(' ').collect::<Vec<&str>>().as_slice() {
                ["addx", count] => Instruction::Add(i32::from_str(count).unwrap()),
                _ => NoOp,
            },
        )
        .collect()
}

#[test]
pub fn test() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    let expected = 13140;
    assert_eq!(expected, problem_1(input.to_string()));

    let expected = "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     ";
    println!("expected:\n{}", &input);
    let problem2 = problem_2(input.to_string());
    println!("actual:\n{}", &problem2);
    assert_eq!(expected, problem2);
}
