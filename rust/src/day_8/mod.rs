use std::str::FromStr;

pub fn run() {
    let input = include_str!("input.txt");
    println!("{:#?}", problem_1(input.to_string()));
    println!("{:#?}", problem_2(input.to_string()));
}

fn problem_1(input: String) -> u32 {
    let mut trees = input
        .lines()
        .map(|line| {
            let tree_line = line
                .chars()
                .map(|c| u32::from_str(c.to_string().as_str()).unwrap_or(0))
                .collect::<Vec<u32>>();

            tree_line
        })
        .collect::<Vec<Vec<u32>>>();

    let mut visible_count = 0;
    let max_row = trees.len() - 1;
    let max_col = trees.first().unwrap().len() - 1;
    for row in 1..max_row {
        for col in 1..max_col {
            let tree = trees[row][col];
            let mut is_visible = false;

            for i in (0..col).rev() {
                if trees[row][i] >= tree {
                    break;
                }

                if i == 0 {
                    is_visible = true;
                }
            }

            if is_visible {
                visible_count += 1;
                continue;
            }

            for i in (col + 1)..=max_col {
                if trees[row][i] >= tree {
                    break;
                }

                if i == max_col {
                    is_visible = true;
                }
            }

            if is_visible {
                visible_count += 1;
                continue;
            }

            for i in (0..row).rev() {
                if trees[i][col] >= tree {
                    break;
                }

                if i == 0 {
                    is_visible = true;
                }
            }

            if is_visible {
                visible_count += 1;
                continue;
            }

            for i in (row + 1)..=max_row {
                if trees[i][col] >= tree {
                    break;
                }

                if i == max_row {
                    is_visible = true;
                }
            }

            if is_visible {
                visible_count += 1;
            }
        }
    }

    visible_count += (trees.len() * 2) as u32;
    visible_count += (trees.first().unwrap().len() * 2) as u32;
    visible_count -= 4;

    visible_count
}

fn problem_2(input: String) -> u32 {
    let mut trees = input
        .lines()
        .map(|line| {
            let tree_line = line
                .chars()
                .map(|c| u32::from_str(c.to_string().as_str()).unwrap_or(0))
                .collect::<Vec<u32>>();

            tree_line
        })
        .collect::<Vec<Vec<u32>>>();

    let mut scores = vec![];
    let max_row = trees.len();
    let max_col = trees.first().unwrap().len();
    for row in 0..max_row {
        for col in 0..max_col {
            let tree = trees[row][col];

            let mut left = 0;
            for i in (0..col).rev() {
                left += 1;
                if trees[row][i] >= tree {
                    break;
                }
            }

            let mut right = 0;
            for i in (col + 1)..max_col {
                right += 1;
                if trees[row][i] >= tree {
                    break;
                }
            }
            
            let mut up = 0;
            for i in (0..row).rev() {
                up += 1;
                if trees[i][col] >= tree {
                    break;
                }
            }

            let mut down = 0;
            for i in (row + 1)..max_row {
                down += 1;
                if trees[i][col] >= tree {
                    break;
                }
            }
            
            let score = (left * right * up * down) as u32;
            scores.push(score)
        }
    }

    scores.sort();
    
    *scores.last().unwrap()
}

#[test]
pub fn test() {
    let input = "30373
25512
65332
33549
35390";
    let expected = 21;
    assert_eq!(problem_1(input.to_string()), expected);

    let expected = 8;
    assert_eq!(problem_2(input.to_string()), expected);
}
