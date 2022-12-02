use std::{fs::File, io::{BufReader, BufRead}};

#[derive(PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors
}
#[derive(PartialEq, Debug)]
enum Outcome {
    Lost,
    Draw,
    Win
}

fn main() -> std::io::Result<()> {
    println!("Advent of Code 2022 - 2");

    let f = File::open("./inputs/02/input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    let mut total_score: u32 = 0;
    loop {
        line.clear();
        let read_result = reader.read_line(&mut line);
        match read_result {
            Ok(0) => { break; },
            Err(err) => {
                println!("Error while reading file");
                return Err(err);
            }
            Ok(_bytes) => {
                let trimmed_line = line.trim_end();
                let plays = parse_plays(&trimmed_line);
                let outcome = get_outcome(&plays);
                let score = count_score(&plays[1], &outcome);
                // println!("Player won: {:?}, score: {}", outcome, score);
                total_score += score;
            }
        }

    }

    println!("Total score: {}", total_score); 

    Ok(())
}

fn parse_plays(input: &str) -> [Play; 2] {
    let play_inputs: Vec<&str> = input.split(" ").collect();
    let player_1 = char_to_play(play_inputs[0]);
    let player_2 = char_to_play(play_inputs[1]);

    [player_1, player_2]
}

fn char_to_play(input: &str) -> Play {
    match input {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => panic!("Unknown play")
    }
}

fn get_outcome(play: &[Play; 2]) -> Outcome {
    if play[0].eq(&play[1]) {
        return Outcome::Draw;
    }
    let has_won = match play[0] {
        Play::Rock => play[1].eq(&Play::Paper),
        Play::Paper => play[1].eq(&Play::Scissors),
        Play::Scissors => play[1].eq(&Play::Rock),
    };

    if has_won {
        Outcome::Win
    } 
    else {
        Outcome::Lost
    }
}

fn count_score(player_play: &Play, outcome: &Outcome) -> u32 {
    let mut score = match player_play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };

    score += match outcome {
        Outcome::Lost => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };

    score
}