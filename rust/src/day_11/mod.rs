use std::str::FromStr;

pub fn run() {
    let input = include_str!("input.txt");
    println!("problem 1: {}", problem_1(input.to_string()));
    println!("problem 2: {}", problem_2(input.to_string()));
}

fn problem_1(input: String) -> u64 {
    let (mut monkeys, _) = parse(input);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for item in 0..monkeys[i].items.len() {
                monkeys[i].inspection_count += 1;
                let worry_level = monkeys[i].inspect(monkeys[i].items[item]) / 3;

                let new_monkey = if monkeys[i].test(worry_level) {
                    monkeys[i].pass_monkey
                } else {
                    monkeys[i].fail_monkey
                };

                monkeys[new_monkey].items.push(worry_level);
            }

            monkeys[i].items.clear();
        }
    }

    let mut counts = monkeys
        .into_iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<u64>>();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

fn problem_2(input: String) -> u64 {
    let ( mut monkeys, cap ) = parse(input);
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for item in 0..monkeys[i].items.len() {
                monkeys[i].inspection_count += 1;
                let worry_level = monkeys[i].inspect(monkeys[i].items[item]) % cap;

                let new_monkey = if monkeys[i].test(worry_level) {
                    monkeys[i].pass_monkey
                } else {
                    monkeys[i].fail_monkey
                };

                monkeys[new_monkey].items.push(worry_level);
            }

            monkeys[i].items.clear();
        }
    }

    let mut counts = monkeys
        .into_iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<u64>>();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
    AddOld,
    MultiplyOld,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    test_value: u64,
    op_value: u64,
    pass_monkey: usize,
    fail_monkey: usize,
    op: Op,
    inspection_count: u64,
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            items: vec![],
            test_value: 0,
            op_value: 0,
            pass_monkey: 0,
            fail_monkey: 0,
            op: Op::Add,
            inspection_count: 0,
        }
    }
    pub fn test(&self, value: u64) -> bool {
        value % self.test_value == 0
    }

    pub fn inspect(&self, value: u64) -> u64 {
        match self.op {
            Op::Add => value + self.op_value,
            Op::Multiply => value * self.op_value,
            Op::MultiplyOld => value * value,
            Op::AddOld => value * value,
        }
    }
}

fn parse(input: String) -> (Vec<Monkey>, u64) {
    let mut cap = 1;
    (
        input
            .split("\n\n")
            .map(|input| {
                let mut monkey = Monkey::new();
                for line in input.lines() {
                    match line.trim().split(' ').collect::<Vec<&str>>().as_slice() {
                        ["Starting", _, items @ ..] => {
                            monkey.items = items
                                .iter()
                                .map(|x| {
                                    u64::from_str(x.split(',').collect::<Vec<&str>>()[0]).unwrap()
                                })
                                .collect();
                        }
                        ["Operation:", _, _, _, "*", "old"] => {
                            monkey.op = Op::MultiplyOld;
                        }
                        ["Operation:", _, _, _, "+", "old"] => {
                            monkey.op = Op::AddOld;
                        }
                        ["Operation:", _, _, _, "*", val] => {
                            monkey.op = Op::Multiply;
                            monkey.op_value = u64::from_str(val).unwrap();
                        }
                        ["Operation:", _, _, _, "+", val] => {
                            monkey.op = Op::Add;
                            monkey.op_value = u64::from_str(val).unwrap();
                        }
                        ["Test:", _, _, val] => {
                            monkey.test_value = u64::from_str(val).unwrap();
                            cap *= monkey.test_value;
                        }
                        [_, "true:", _, _, _, val] => {
                            monkey.pass_monkey = usize::from_str(val).unwrap();
                        }
                        [_, "false:", _, _, _, val] => {
                            monkey.fail_monkey = usize::from_str(val).unwrap();
                        }
                        &_ => {}
                    }
                }
                monkey
            })
            .collect(),
        cap,
    )
}

#[test]
pub fn test_parse() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    println!("{:#?}", parse(input.to_string()))
}

#[test]
pub fn test() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    let expected = 10605;
    assert_eq!(expected, problem_1(input.to_string()));

    let expected = 2713310158;
    assert_eq!(expected, problem_2(input.to_string()));
}
