use crate::day_13::PacketValue::{List, Val};
use nom::branch::alt;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use std::cmp::Ordering;

pub fn run() {
    let input = include_str!("input.txt");
    let result = problem_1(input.to_string());
    dbg!(result);
    let result = problem_2(input.to_string());
    dbg!(result);
}

fn problem_1(input: String) -> u32 {
    let groups = parse(input);
    groups.into_iter().enumerate().fold(0, |agg, (idx, group)| {
        if group.is_right_order() {
            dbg!(idx);
            agg + idx as u32 + 1
        } else {
            agg
        }
    })
}

fn problem_2(input: String) -> u32 {
    let mut packets = parse(input)
        .into_iter()
        .flat_map(|group| vec![group.left, group.right])
        .collect::<Vec<PacketValue>>();
    let separator_one = List(vec![List(vec![Val(2)])]);
    let separator_two = List(vec![List(vec![Val(6)])]);
    packets.push(separator_one.clone());
    packets.push(separator_two.clone());
    packets.sort();

    packets
        .into_iter()
        .enumerate()
        .fold(1, |agg, (idx, packet)| {
            if packet == separator_one || packet == separator_two {
                agg * ( idx as u32 + 1)
            } else {
                agg
            }
        })
}

fn parse(input: String) -> Vec<PacketGroup> {
    input
        .split("\n\n")
        .map(|group| {
            let packets = group
                .lines()
                .map(|line| parse_list(line).unwrap().1)
                .collect::<Vec<PacketValue>>();

            PacketGroup {
                left: packets[0].clone(),
                right: packets[1].clone(),
            }
        })
        .collect()
}

fn parse_val(input: &str) -> IResult<&str, PacketValue> {
    let (input, raw_val) = complete::u32(input)?;
    Ok((input, Val(raw_val)))
}

fn parse_list(input: &str) -> IResult<&str, PacketValue> {
    let (input, result) = delimited(
        complete::char('['),
        separated_list0(complete::char(','), alt((parse_val, parse_list))),
        complete::char(']'),
    )(input)?;

    Ok((input, List(result)))
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct PacketGroup {
    left: PacketValue,
    right: PacketValue,
}

impl PacketGroup {
    pub fn is_right_order(&self) -> bool {
        self.left.is_before(&self.right)
    }
}

impl PartialOrd<Self> for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_before(other) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum PacketValue {
    List(Vec<PacketValue>),
    Val(u32),
}

impl PacketValue {
    pub fn is_before(&self, other: &PacketValue) -> bool {
        match self {
            List(values) => match other {
                List(other_values) => {
                    for i in 0..values.len() {
                        if i == other_values.len() {
                            return false;
                        }

                        if values[i] == other_values[i] {
                            continue;
                        }

                        return values[i].is_before(&other_values[i]);
                    }

                    true
                }
                Val(other_val) => self.is_before(&List(vec![Val(*other_val)])),
            },
            Val(value) => match other {
                List(_) => List(vec![Val(*value)]).is_before(other),
                Val(other_val) => value <= other_val,
            },
        }
    }
}

#[test]
fn test_parse() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    println!("{:#?}", parse(input.to_string()))
}

#[test]
fn test() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    let expected = 13;
    assert_eq!(expected, problem_1(input.to_string()));

    let expected = 140;
    assert_eq!(expected, problem_2(input.to_string()));
}
