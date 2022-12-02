use std::{fs::File, io::{BufReader, BufRead}};

#[derive(PartialEq, Clone, Copy)]
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
                
                let round = parse_round(&trimmed_line);
                let my_play = match_play(&round);
                let score = count_score(&my_play, &round.1);
                // println!("Player won: {:?}, score: {}", outcome, score);
                total_score += score;
            }
        }

    }

    println!("Total score: {}", total_score); 

    Ok(())
}

fn parse_round(input: &str) -> (Play, Outcome) {
    let play_inputs: Vec<&str> = input.split(" ").collect();
    let opponent = char_to_play(play_inputs[0]);
    let outcome = char_to_outcome(play_inputs[1]);

    (opponent, outcome)
}

fn char_to_play(input: &str) -> Play {
    match input {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => panic!("Unknown play")
    }
}

fn char_to_outcome(input: &str) -> Outcome {
    match input {
        "X" => Outcome::Lost,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unknown outcome")
    }
}

fn match_play(play: &(Play, Outcome)) -> Play {
    if play.1.eq(&Outcome::Draw) {
        return play.0;
    }
    match play.0 {
        Play::Rock => if play.1.eq(&Outcome::Win) {
            Play::Paper
        }
        else {
            Play::Scissors
        },
        Play::Paper => if play.1.eq(&Outcome::Win) {
            Play::Scissors
        }
        else {
            Play::Rock
        },
        Play::Scissors => if play.1.eq(&Outcome::Win) {
            Play::Rock
        }
        else {
            Play::Paper
        },
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