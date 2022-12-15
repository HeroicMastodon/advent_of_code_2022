use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list0;
use nom::IResult;
use std::collections::{BTreeSet, HashSet, VecDeque};

pub fn run() {
    let input = include_str!("input.txt");
    let result = problem_1(input);
    dbg!(result);
    let result = problem_2(input);
    dbg!(result);
}

pub fn problem_2(input: &str) -> u32 {
    let paths = parse(input);
    let (lower_limit, upper_limit) = (&paths).iter().fold(
        (Point::new(usize::MAX, 0), Point::new(0, 0)),
        |(lower_limit, upper_limit), path| {
            let mut min_x = lower_limit.x;
            let mut max_x = upper_limit.x;
            let mut max_y = upper_limit.y;

            for point in path {
                min_x = usize::min(point.x, min_x);
                max_x = usize::max(point.x, max_x);
                max_y = usize::max(point.y, max_y);
            }

            (Point::new(min_x, 0), Point::new(max_x, max_y))
        },
    );

    let upper_limit = Point::new(upper_limit.x, upper_limit.y + 2);
    let mut grid = vec![vec!['.'; upper_limit.x * 2]; upper_limit.y];
    grid.push(vec!['#'; upper_limit.x * 2]);

    for path in paths {
        let mut prev = None;
        for point in path {
            match prev {
                None => {
                    prev = Some(point);
                    grid[point.y][point.x] = '#';
                }
                Some(prev_point) => {
                    if prev_point.x != point.x {
                        let mut x_range = prev_point.x..=point.x;
                        if prev_point.x > point.x {
                            x_range = point.x..=prev_point.x;
                        }

                        for x in x_range {
                            grid[point.y][x] = '#';
                        }
                    }

                    if prev_point.y != point.y {
                        let mut y_range = prev_point.y..=point.y;
                        if prev_point.y > point.y {
                            y_range = point.y..=prev_point.y;
                        }

                        for y in y_range {
                            grid[y][point.x] = '#';
                        }
                    }
                    prev = Some(point);
                }
            }
        }
    }

    let start_point = Point::new(500, 0);
    let mut sand_count = 0;
    let mut sand_point = start_point;
    let mut iter_count = 0;

    loop {
        iter_count += 1;
        let down = Point::new(sand_point.x, sand_point.y + 1);
        if down.y > upper_limit.y {
            grid[sand_point.y][sand_point.x] = 'o';
            sand_point = start_point;
            continue;
        }
        
        if grid[down.y][down.x] == '.' {
            sand_point = down;
            continue;
        }

        let left = Point::new(sand_point.x.wrapping_sub(1), sand_point.y + 1);
        if left.x == usize::MAX {
            panic!("x rolled over")
        }
        
        if grid[left.y][left.x] == '.' {
            sand_point = left;
            continue;
        }

        let right = Point::new(sand_point.x + 1, sand_point.y + 1);
        if grid[right.y][right.x] == '.' {
            sand_point = right;
            continue;
        }

        sand_count += 1;
        grid[sand_point.y][sand_point.x] = 'o';
        if sand_point == start_point {
            break;
        }
        sand_point = start_point;
    }
    
    dbg!(iter_count);
    // print_grid(&grid);

    sand_count
}

pub fn print_grid(grid: &[Vec<char>]) {
    println!(
        "{}",
        grid.iter()
            .map(|line| line.iter().fold("".to_string(), |mut str, point| {
                str.push(*point);
                str
            }) + "\n")
            .collect::<String>()
    );
}

pub fn problem_1(input: &str) -> u32 {
    let paths = parse(input);
    let (lower_limit, upper_limit) = (&paths).iter().fold(
        (Point::new(usize::MAX, 0), Point::new(0, 0)),
        |(lower_limit, upper_limit), path| {
            let mut min_x = lower_limit.x;
            let mut max_x = upper_limit.x;
            let mut max_y = upper_limit.y;

            for point in path {
                min_x = usize::min(point.x, min_x);
                max_x = usize::max(point.x, max_x);
                max_y = usize::max(point.y, max_y);
            }

            (Point::new(min_x, 0), Point::new(max_x, max_y))
        },
    );
    let paths = paths
        .into_iter()
        .map(|path| {
            path.iter()
                .map(|point| Point::new(point.x - lower_limit.x, point.y))
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();
    let mut grid = vec![
        vec!['.'; upper_limit.x as usize - lower_limit.x as usize + 1];
        upper_limit.y as usize + 1
    ];

    for path in paths {
        let mut prev = None;
        for point in path {
            match prev {
                None => {
                    prev = Some(point);
                    grid[point.y][point.x] = '#';
                }
                Some(prev_point) => {
                    if prev_point.x != point.x {
                        let mut x_range = prev_point.x..=point.x;
                        if prev_point.x > point.x {
                            x_range = point.x..=prev_point.x;
                        }

                        for x in x_range {
                            grid[point.y][x] = '#';
                        }
                    }

                    if prev_point.y != point.y {
                        let mut y_range = prev_point.y..=point.y;
                        if prev_point.y > point.y {
                            y_range = point.y..=prev_point.y;
                        }

                        for y in y_range {
                            grid[y][point.x] = '#';
                        }
                    }
                    prev = Some(point);
                }
            }
        }
    }

    let mut sand_count = 0;
    let starting_point = Point::new(500 - lower_limit.x, 0);
    let mut sand_point = starting_point;
    loop {
        let down = Point::new(sand_point.x, sand_point.y + 1);
        if down.y > upper_limit.y {
            break;
        }
        if grid[down.y][down.x] == '.' {
            sand_point = down;
            continue;
        }

        let left = Point::new(sand_point.x.wrapping_sub(1), sand_point.y + 1);
        if left.x == usize::MAX {
            break;
        }
        if grid[left.y][left.x] == '.' {
            sand_point = left;
            continue;
        }

        let right = Point::new(sand_point.x + 1, sand_point.y + 1);
        if right.x > upper_limit.x - lower_limit.x {
            break;
        }
        if grid[right.y][right.x] == '.' {
            sand_point = right;
            continue;
        }

        sand_count += 1;
        grid[sand_point.y][sand_point.x] = 'o';
        sand_point = starting_point;
    }

    sand_count
}

fn parse(input: &str) -> Vec<VecDeque<Point>> {
    let (_, result) = separated_list0(newline, parse_line)(input).unwrap();
    result
}

fn parse_line(input: &str) -> IResult<&str, VecDeque<Point>> {
    let (input, vec) = separated_list0(tag(" -> "), parse_point)(input)?;
    Ok((input, VecDeque::from(vec)))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = complete::u32(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, y) = complete::u32(input)?;

    Ok((
        input,
        Point {
            x: x as usize,
            y: y as usize,
        },
    ))
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[test]
pub fn test_parse() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let expected = vec![
        VecDeque::from(vec![
            Point::new(498, 4),
            Point::new(498, 6),
            Point::new(496, 6),
        ]),
        VecDeque::from(vec![
            Point::new(503, 4),
            Point::new(502, 4),
            Point::new(502, 9),
            Point::new(494, 9),
        ]),
    ];

    assert_eq!(expected, parse(input));
}

#[test]
pub fn test() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    let expected = 24;
    assert_eq!(expected, problem_1(input));
    let expected = 93;
    assert_eq!(expected, problem_2(input));
}
