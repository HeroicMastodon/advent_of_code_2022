use std::str::{FromStr};
use crate::read_lines;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, digit1, multispace1, newline, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

#[derive(Debug)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

fn crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let (input, options) = separated_list1(complete::char(' '), parse_crate)(input)?;
    Ok((input, options))
}

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, parsed_crate) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match parsed_crate {
        "   " => None,
        value => Some(value.chars().next().unwrap()),
    };

    Ok((input, result))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;
    Ok((
        input,
        Move {
            count,
            to: to - 1,
            from: from - 1,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Move>)> {
    let (input, crate_options) = separated_list1(newline, crate_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, parse_move)(input)?;

    let mut crates: Vec<Vec<char>> = vec![];
    for _ in 0..=crate_options.len() {
        crates.push(vec![]);
    }

    for option_list in crate_options.iter().rev() {
        for (idx, c) in option_list.iter().enumerate() {
            match c {
                None => {}
                Some(label) => crates[idx].push(*label),
            }
        }
    }

    Ok((input, (crates, moves)))
}

fn parse_manual(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let (stack_lines, move_lines) = (split[0].lines(), split[1].lines());
    
    let mut stacks: Vec<Vec<char>> = vec![];
    
    for line in stack_lines {
        if stacks.is_empty() {
            for _ in 0..=line.chars().count() / 4 {
                stacks.push(vec![]);
            }
        }
        
        for stack in 0..stacks.len() {
            let chars = line.chars().skip(stack * 4).take(4);
            
            for c in chars {
                if c.is_alphabetic() {
                    stacks.get_mut(stack).unwrap().push(c);
                }
            }
        }
    }

    stacks = stacks.into_iter().map(|mut stack| {stack.reverse(); stack}).collect();
    
    let moves = move_lines.map(|line| {
        let words = line.split(" ").collect::<Vec<&str>>();
        
        Move {
            count: u32::from_str(words[1]).unwrap(),
            from: u32::from_str(words[3]).unwrap() - 1,
            to: u32::from_str(words[5]).unwrap() - 1
        }
    });

    (stacks, moves.collect())
}

pub fn problem_1() {
    let file = read_lines("src/day_5/input.txt").unwrap();
    let _input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    let (mut crates, moves) = parse_manual(file.as_str());

    for Move { count, from, to } in moves.into_iter() {
        for _ in 0..count {
            let c = crates.get_mut(from as usize).unwrap().pop();
            crates
                .get_mut(to as usize)
                .unwrap()
                .push(c.unwrap());
        }
    }

    let mut tops: Vec<char> = vec![];
    for c in crates.into_iter() {
        match c.last() {
            None => {}
            Some(top) => {
                tops.push(*top);
            }
        }
    }
    println!("{:#?}", tops.into_iter().collect::<String>());
}

pub fn problem_2() {
    let file = read_lines("src/day_5/input.txt").unwrap();
    let _input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    let (_input, (mut crates, moves)) = parse(file.as_str()).unwrap();

    for Move { count, from, to } in moves.into_iter() {
        let mut crates_to_move: Vec<char> = vec![];
        let mut stack = crates.get_mut(from as usize).unwrap();
        
        for _ in 0..count {
            let c = stack.pop();
            crates_to_move.push(c.unwrap());
        }
        
        crates_to_move.reverse();
        
        stack = crates.get_mut(to as usize).unwrap();
        for c in crates_to_move.into_iter() {
            stack.push(c);
        }
    }

    let mut tops: Vec<char> = vec![];
    for c in crates.into_iter() {
        match c.last() {
            None => {}
            Some(top) => {
                tops.push(*top);
            }
        }
    }
    println!("{:#?}", tops.into_iter().collect::<String>());
    // let (crates, moves) = parse(file.into());
}
