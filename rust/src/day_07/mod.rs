use std::collections::HashMap;
use std::str::FromStr;

struct DirFile {
    name: String,
    size: u32,
}

struct Dir {
    name: String,
    files: Vec<DirFile>,
}

pub fn run() {
    let input = include_str!("input.txt");
    println!("{:#?}", problem_1(input.to_string()));
    println!("{:#?}", problem_2(input.to_string()));
}

pub fn problem_1(input: String) -> u32 {
    let directory_sizes = process_input(input);

    let mut sum = 0;
    for (_, val) in directory_sizes {
        if val <= 100000 {
            sum += val;
        }
    }

    sum
}

pub fn problem_2(input: String) -> u32 {
    let directory_sizes = process_input(input);
    let total_space = 70000000;
    let update_size = 30000000;
    let remaining_space = total_space - directory_sizes.get("/").unwrap_or(&0);
    let needed_space = update_size - remaining_space;
    let mut filtered = directory_sizes
        .into_iter()
        .map(|(_, val)| val)
        .filter(|val| val >= &needed_space)
        .collect::<Vec<u32>>();

    filtered.sort();

    return *filtered.first().unwrap_or(&0);
}

fn process_input(input: String) -> HashMap<String, u32> {
    let mut curr_dir = String::new();
    let mut directory_sizes = HashMap::new();
    let mut dir_stack = vec![];
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["$", "cd", "/"] => {
                curr_dir = "/".to_string();
                if !directory_sizes.contains_key("/") {
                    directory_sizes.insert("/".to_string(), 0);
                }
                dir_stack.clear();
            }
            ["$", "cd", ".."] => {
                curr_dir = dir_stack.pop().unwrap();
            }
            ["$", "cd", dir] => {
                dir_stack.push(curr_dir.clone());
                curr_dir = curr_dir.clone() + dir + "/";
            }
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, _] => {
                *directory_sizes.entry(curr_dir.clone()).or_insert(0) +=
                    u32::from_str(size).unwrap_or(0);
                for dir in &dir_stack {
                    *directory_sizes.entry(dir.clone()).or_insert(0) +=
                        u32::from_str(size).unwrap_or(0);
                }
            }
            _ => {}
        };
    }
    directory_sizes
}

#[test]
pub fn test_day_7() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    let expected = 95437;
    assert_eq!(expected, problem_1(input.to_string()));

    let expected = 24933642;
    assert_eq!(expected, problem_2(input.to_string()));
}
