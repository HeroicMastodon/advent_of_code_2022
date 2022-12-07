use crate::{BASE_URL, read_lines};

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOSS: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

pub fn day_2() {
    let file_name = "src/day_2/input.txt";
    let file = read_lines(file_name).unwrap_or("".to_string());

    let result: u32 = file.split("\n").map(score_game).sum();

    println!("{result}")
}

fn score_game(game: &str) -> u32 {
    let moves_vec: Vec<&str> = game.split(' ').collect();
    let moves = (moves_vec[0], moves_vec[1]);

    match moves {
        // Rock
        ("A", "X") => SCISSORS + LOSS,
        ("A", "Y") => ROCK + DRAW,
        ("A", "Z") => PAPER + WIN,
        // Paper
        ("B", "X") => ROCK + LOSS,
        ("B", "Y") => PAPER + DRAW,
        ("B", "Z") => SCISSORS + WIN,
        // Scissors
        ("C", "X") => PAPER + LOSS,
        ("C", "Y") => SCISSORS + DRAW,
        ("C", "Z") => ROCK + WIN,
        tuple => {
            println!("missed case: {tuple:#?}");
            0
        }
    }
}