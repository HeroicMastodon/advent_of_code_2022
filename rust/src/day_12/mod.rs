use std::collections::VecDeque;

pub fn run() {
    let input = include_str!("input.txt");
    let result = problem_1(input.to_string());
    dbg!(result);
    let result = problem_2(input.to_string());
    dbg!(result);
}

fn problem_1(input: String) -> u32 {
    let (grid, start, end, limits, _) = parse(input);
    let result = shortest_path(&grid, start, &end, &limits);

    result.unwrap()
}

fn problem_2(input: String) -> u32 {
    let (grid, _start, end, limits, lowest_points) = parse(input);
    lowest_points
        .into_iter()
        .map(|point| shortest_path(&grid, point, &end, &limits).unwrap_or(u32::MAX))
        .min()
        .unwrap()
}

fn valid_destination(curr: char, destination: char) -> bool {
    if !destination.is_alphabetic() || destination == 'V' {
        return false;
    }

    let curr_val = if curr == 'S' { 'a' as i32 } else { curr as i32 };
    let dest_val = if destination == 'E' {
        'z' as i32
    } else if destination == 'S' {
        'a' as i32
    } else {
        destination as i32
    };

    let diff = dest_val - curr_val;
    diff <= 1
}

fn shortest_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    destination: &(usize, usize),
    limits: &(usize, usize),
) -> Option<u32> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (curr, count) = queue.pop_front().unwrap();

        if curr.0 == destination.0 && curr.1 == destination.1 {
            return Some(count);
        }

        let curr_elev = grid[curr.0][curr.1];
        let mut update_neighbor = |row: usize, col: usize| {
            if row >= limits.0
                || col >= limits.1
                || !valid_destination(curr_elev, grid[row][col])
                || visited[row][col]
            {
                return;
            }

            visited[row][col] = true;
            queue.push_back(((row, col), count + 1));
        };

        update_neighbor(curr.0 + 1, curr.1);
        update_neighbor(curr.0.wrapping_sub(1), curr.1);
        update_neighbor(curr.0, curr.1 + 1);
        update_neighbor(curr.0, curr.1.wrapping_sub(1));
    }

    None
}

fn parse(
    input: String,
) -> (
    Vec<Vec<char>>,
    (usize, usize),
    (usize, usize),
    (usize, usize),
    Vec<(usize, usize)>,
) {
    let mut start: (usize, usize) = (0, 0);
    let mut limits: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut line_number = 0;
    let mut lowest_points = vec![];
    let grid = input
        .lines()
        .map(|line| {
            if limits.1 == 0 {
                limits.1 = line.len();
            }

            if let Some(start_index) = line.chars().position(|c| c == 'S') {
                start = (line_number, start_index);
            }

            for (idx, c) in line.chars().enumerate() {
                if c == 'E' {
                    end = (line_number, idx);
                }

                if c == 'S' {
                    start = (line_number, idx);
                    lowest_points.push((line_number, idx));
                }

                if c == 'a' {
                    lowest_points.push((line_number, idx));
                }
            }

            line_number += 1;
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    limits.0 = grid.len();

    (grid, start, end, limits, lowest_points)
}
#[test]
fn test_can_move() {
    assert!(valid_destination('S', 'a'));
    assert!(valid_destination('S', 'b'));
    assert!(!valid_destination('S', 'c'));
    assert!(!valid_destination('S', '<'));
    assert!(!valid_destination('S', '>'));
    assert!(!valid_destination('S', '^'));
    assert!(!valid_destination('S', 'V'));
    assert!(valid_destination('z', 'E'));
    assert!(valid_destination('y', 'E'));
    assert!(!valid_destination('v', 'E'));
    assert!(valid_destination('a', 'S'));
    assert!(valid_destination('b', 'S'));
}

#[test]
fn test_parse() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    println!("{:#?}", parse(input.to_string()))
}

#[test]
fn test() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let expected = 31;
    assert_eq!(expected, problem_1(input.to_string()));

    let expected = 29;
    assert_eq!(expected, problem_2(input.to_string()));
}
