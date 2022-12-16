use crate::day_09::Move::{Left, Right};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(PartialOrd, PartialEq, Debug, Hash, Clone, Ord, Eq)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
    pub fn is_touching(&self, other: &Position) -> bool {
        if self.x == other.x && self.y == other.y {
            true
        } else {
            self.distance_to(other) <= f64::sqrt(2_f64)
        }
    }

    pub fn new_touching_point(&self, other: &Position) -> Position {
        let mut new_position = Position::new(other.x, other.y);
        new_position.y += if self.is_above(other) {
            1
        } else if self.is_below(other) {
            -1
        } else {
            0
        };

        new_position.x += if self.is_right(other) {
            1
        } else if self.is_left(other) {
            -1
        } else {
            0
        };

        new_position
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        f64::sqrt(((self.y - other.y).pow(2) + (self.x - other.x).pow(2)) as f64)
    }

    pub fn is_left(&self, other: &Position) -> bool {
        self.x < other.x
    }
    pub fn is_right(&self, other: &Position) -> bool {
        self.x > other.x
    }
    pub fn is_above(&self, other: &Position) -> bool {
        self.y > other.y
    }
    pub fn is_below(&self, other: &Position) -> bool {
        self.y < other.y
    }
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn from_str(input: String) -> (Move, i32) {
        match input.split(' ').collect::<Vec<&str>>().as_slice() {
            ["R", count] => (Right, i32::from_str(count).unwrap()),
            ["L", count] => (Left, i32::from_str(count).unwrap()),
            ["U", count] => (Move::Up, i32::from_str(count).unwrap()),
            ["D", count] => (Move::Down, i32::from_str(count).unwrap()),
            invalid_move => panic!("invalid move: {:#?}", invalid_move),
        }
    }
}

pub fn run() {
    let input = include_str!("input.txt");
    println!("{:#?}", problem_1(input.to_string()));
    println!("{:#?}", problem_2(input.to_string()));
}

fn parse_moves(input: String) -> Vec<(Move, i32)> {
    input
        .lines()
        .map(|line| Move::from_str(line.to_string()))
        .collect()
}

fn problem_2(input: String) -> u32 {
    let moves = parse_moves(input);
    let mut tail_locations: HashSet<Position> = HashSet::new();
    let mut rope = vec![Position::new(0,0); 10];

    for (direction, distance) in moves {
        for _ in 0..distance {
            let mut head = rope.get_mut(0).unwrap();
            match direction {
                Move::Up => {
                    head.y += 1;
                }
                Move::Down => {
                    head.y -= 1;
                }
                Left => {
                    head.x -= 1;
                }
                Right => {
                    head.x += 1;
                }
            }
            for i in 0..9 {
                let follower = rope.get(i + 1).unwrap();
                let leader = rope.get(i).unwrap();

                if !leader.is_touching(follower) {
                    rope[i + 1] =  leader.new_touching_point(follower);
                }
            }

            tail_locations.insert(rope.last().unwrap().clone());
        }
    }

    tail_locations.len() as u32
}

fn problem_1(input: String) -> u32 {
    let moves = parse_moves(input);
    let mut tail_locations: HashSet<Position> = HashSet::new();
    let mut tail = Position::new(0, 0);
    let mut head = Position::new(0, 0);

    for (direction, distance) in moves {
        for _ in 0..distance {
            let prev_head = head.clone();
            match direction {
                Move::Up => {
                    head.y += 1;
                }
                Move::Down => {
                    head.y -= 1;
                }
                Left => {
                    head.x -= 1;
                }
                Right => {
                    head.x += 1;
                }
            }

            if !head.is_touching(&tail) {
                tail = prev_head;
            }

            tail_locations.insert(tail.clone());
        }
    }

    tail_locations.len() as u32
}

#[test]
fn test() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let expected = 13;
    assert_eq!(problem_1(input.to_string()), expected);

    let expected = 1;
    assert_eq!(problem_2(input.to_string()), expected);
    
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let expected = 36;
    assert_eq!(problem_2(input.to_string()), expected);
}
