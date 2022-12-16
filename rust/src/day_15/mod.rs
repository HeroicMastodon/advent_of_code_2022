use crate::day_15::PointType::{Beacon, Invalid, Sensor};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list0;
use nom::IResult;
use std::collections::HashSet;

pub fn run() {
    let input = include_str!("input.txt");
    let target_y = 2000000;
    let result = problem_1(input, target_y);
    dbg!(result);
    let result = problem_2(input, 4000000);
    dbg!(result);
}

pub fn problem_1(input: &str, target_y: i32) -> u32 {
    let pairs = parse(input);
    let mut lower_limit = i32::MAX;
    let mut upper_limit = i32::MIN;
    let mut beacon_count = 0;

    for (sensor, beacon) in &pairs {
        let distance_from_beacon = sensor.distance(beacon);

        if !is_in_range(target_y, sensor, distance_from_beacon) {
            continue;
        }

        let distance_from_target = sensor.y.abs_diff(target_y);
        let offset = (distance_from_beacon as i32 - distance_from_target as i32).abs();
        lower_limit = (sensor.x - offset).min(lower_limit);
        upper_limit = (sensor.x + offset).max(upper_limit);
    }

    let unique_beacons = (&pairs)
        .iter()
        .map(|(_, beacon)| beacon)
        .cloned()
        .collect::<HashSet<Point>>();

    for beacon in &unique_beacons {
        if beacon.y == target_y && lower_limit <= beacon.x && beacon.x <= upper_limit {
            beacon_count += 1;
        }
    }

    (upper_limit - lower_limit - beacon_count + 1) as u32
}

pub fn problem_2(input: &str, limit: usize) -> u128 {
    let pairs = parse(input);
    let mut target = None;

    for (sensor, beacon) in (&pairs).iter() {
        let y_offset = sensor.distance(beacon) + 1;

        for y in
            (sensor.y - y_offset as i32).max(0)..=(sensor.y + y_offset as i32).min(limit as i32)
        {
            let x_offset = (y.abs_diff(sensor.y)).abs_diff(y_offset as u32) as i32;

            let left = sensor.x - x_offset;
            let left_point = Point::invalid(left, y);
            if left >= 0 && left <= limit as i32 && !is_sensed(&pairs, left_point) {
                target = Some(left_point);
                break;
            }

            let right = sensor.x + x_offset;
            let right_point = Point::invalid(right, y);
            if right <= limit as i32 && right >= 0 && !is_sensed(&pairs, right_point) {
                target = Some(right_point);
                break;
            }
        }

        if target.is_some() {
            break;
        }
    }

    let point = target.unwrap();

    (point.x as u128 * 4000000 + point.y as u128) as u128
}

fn is_beacon(pairs: &[(Point, Point)], point: &Point) -> bool {
    pairs.iter().any(|(_, beacon)| beacon == point)
}

fn is_in_range(target: i32, start: &Point, range: u32) -> bool {
    let upper_limit = start.y + range as i32;
    let lower_limit = start.y - range as i32;

    lower_limit <= target && target <= upper_limit
}

fn print_grid(grid: &[Vec<PointType>]) {
    println!(
        "{}",
        grid.iter()
            .enumerate()
            .map(|(idx, line)| {
                let str = line.iter().fold("".to_string(), |mut str, point| {
                    let c = match point {
                        Sensor => 'S',
                        Beacon => 'B',
                        Invalid => '#',
                        PointType::None => '.',
                    };
                    str.push(c);
                    str
                }) + "\n";

                format!("{:0>2} {}", idx, str)
            })
            .collect::<String>()
    );
}

fn is_sensed(pairs: &[(Point, Point)], point: Point) -> bool {
    pairs
        .iter()
        .any(|(sensor, beacon)| sensor.distance(beacon) >= sensor.distance(&point))
}

#[test]
fn test_sensed() {
    let input = "sensor at x=2, y=18: closest beacon is at x=-2, y=15
sensor at x=9, y=16: closest beacon is at x=10, y=16
sensor at x=13, y=2: closest beacon is at x=15, y=3
sensor at x=12, y=14: closest beacon is at x=10, y=16
sensor at x=10, y=20: closest beacon is at x=10, y=16
sensor at x=14, y=17: closest beacon is at x=10, y=16
sensor at x=8, y=7: closest beacon is at x=2, y=10
sensor at x=2, y=0: closest beacon is at x=2, y=10
sensor at x=0, y=11: closest beacon is at x=2, y=10
sensor at x=20, y=14: closest beacon is at x=25, y=17
sensor at x=17, y=20: closest beacon is at x=21, y=22
sensor at x=16, y=7: closest beacon is at x=15, y=3
sensor at x=14, y=3: closest beacon is at x=15, y=3
sensor at x=20, y=1: closest beacon is at x=15, y=3";
    let pairs = parse(input);

    assert!(!is_sensed(&pairs, Point::invalid(14, 11)));
}

fn parse(input: &str) -> Vec<(Point, Point)> {
    let (_, result) = separated_list0(newline, parse_line)(input).unwrap();
    result
}

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    let (input, points) = separated_list0(tag(": "), alt((parse_sensor, parse_beacon)))(input)?;
    Ok((input, (points[0], points[1])))
}

fn parse_sensor(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("sensor at x=")(input)?;
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = complete::i32(input)?;

    Ok((input, Point::sensor(x, y)))
}
fn parse_beacon(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("closest beacon is at x=")(input)?;
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = complete::i32(input)?;

    Ok((input, Point::beacon(x, y)))
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
    point_type: PointType,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd, Hash)]
enum PointType {
    Sensor,
    Beacon,
    Invalid,
    None,
}

impl Point {
    pub fn sensor(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
            point_type: Sensor,
        }
    }
    pub fn beacon(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
            point_type: Beacon,
        }
    }
    pub fn invalid(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
            point_type: Invalid,
        }
    }
    pub fn distance(&self, right: &Point) -> u32 {
        Point::_distance(self.x, self.y, right.x, right.y)
    }

    fn _distance(left_x: i32, left_y: i32, right_x: i32, right_y: i32) -> u32 {
        left_x.abs_diff(right_x) + left_y.abs_diff(right_y)
    }
}

#[test]
pub fn test_parse() {
    let input = "sensor at x=2, y=18: closest beacon is at x=-2, y=15
sensor at x=9, y=16: closest beacon is at x=10, y=16
sensor at x=13, y=2: closest beacon is at x=15, y=3
sensor at x=12, y=14: closest beacon is at x=10, y=16
sensor at x=10, y=20: closest beacon is at x=10, y=16
sensor at x=14, y=17: closest beacon is at x=10, y=16
sensor at x=8, y=7: closest beacon is at x=2, y=10
sensor at x=2, y=0: closest beacon is at x=2, y=10
sensor at x=0, y=11: closest beacon is at x=2, y=10
sensor at x=20, y=14: closest beacon is at x=25, y=17
sensor at x=17, y=20: closest beacon is at x=21, y=22
sensor at x=16, y=7: closest beacon is at x=15, y=3
sensor at x=14, y=3: closest beacon is at x=15, y=3
sensor at x=20, y=1: closest beacon is at x=15, y=3";
    let expected = vec![
        (Point::sensor(2, 18), (Point::beacon(-2, 15))),
        (Point::sensor(9, 16), (Point::beacon(10, 16))),
        (Point::sensor(13, 2), (Point::beacon(15, 3))),
        (Point::sensor(12, 14), (Point::beacon(10, 16))),
        (Point::sensor(10, 20), (Point::beacon(10, 16))),
        (Point::sensor(14, 17), (Point::beacon(10, 16))),
        (Point::sensor(8, 7), (Point::beacon(2, 10))),
        (Point::sensor(2, 0), (Point::beacon(2, 10))),
        (Point::sensor(0, 11), (Point::beacon(2, 10))),
        (Point::sensor(20, 14), (Point::beacon(25, 17))),
        (Point::sensor(17, 20), (Point::beacon(21, 22))),
        (Point::sensor(16, 7), (Point::beacon(15, 3))),
        (Point::sensor(14, 3), (Point::beacon(15, 3))),
        (Point::sensor(20, 1), (Point::beacon(15, 3))),
    ];

    assert_eq!(expected, parse(input));
}

#[test]
pub fn test() {
    let input = "sensor at x=2, y=18: closest beacon is at x=-2, y=15
sensor at x=9, y=16: closest beacon is at x=10, y=16
sensor at x=13, y=2: closest beacon is at x=15, y=3
sensor at x=12, y=14: closest beacon is at x=10, y=16
sensor at x=10, y=20: closest beacon is at x=10, y=16
sensor at x=14, y=17: closest beacon is at x=10, y=16
sensor at x=8, y=7: closest beacon is at x=2, y=10
sensor at x=2, y=0: closest beacon is at x=2, y=10
sensor at x=0, y=11: closest beacon is at x=2, y=10
sensor at x=20, y=14: closest beacon is at x=25, y=17
sensor at x=17, y=20: closest beacon is at x=21, y=22
sensor at x=16, y=7: closest beacon is at x=15, y=3
sensor at x=14, y=3: closest beacon is at x=15, y=3
sensor at x=20, y=1: closest beacon is at x=15, y=3";

    let expected = 26;
    let actual = problem_1(input, 10);
    assert_eq!(expected, actual);
    let expected = 56000011;
    assert_eq!(expected, problem_2(input, 20));
}
