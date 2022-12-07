use std::collections::HashMap;
use std::fmt;

pub mod data;

#[derive(Clone, Debug, PartialEq)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, PartialEq)]
enum RpsPlay {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for RpsPlay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

fn main() {
    let round_scores = read_to_round_scores();
    let num_of_rounds = round_scores.iter().count();
    let sum: u16 = round_scores.iter().sum();

    println!("The Sum of All Rounds ({num_of_rounds}) is: {sum}");
}

fn read_to_round_scores() -> Vec::<u16>{
    let rps_map = HashMap::from([
        ("A", RpsPlay::Rock),
        ("B", RpsPlay::Paper),
        ("C", RpsPlay::Scissors),
        ("X", RpsPlay::Rock),
        ("Y", RpsPlay::Paper),
        ("Z", RpsPlay::Scissors),
    ]);

    let mut round_score = Vec::<u16>::new();
    if let Ok(lines) = data::read_data_from_file() {
        for line in lines {
            if let Ok(round_plays) = line {
                if !round_plays.is_empty() {
                    let plays = split_line_into_plays(round_plays);
                    let other_play = rps_map.get(plays[0].as_str()).expect("Missing Other Play");
                    let my_play = rps_map.get(plays[1].as_str()).expect("Missing My Play");
                    let result = determine_round_result(other_play, my_play);
                    let score = calculate_round_value(my_play, result);
                    // println!("Other Play: {other_play}\tMy Play: {my_play}\t\tScore: {score}");
                    round_score.push(score);
                }
            }
        }
    }
    // Return
    round_score
}

fn split_line_into_plays(line: String) -> Vec<String> {
    line.split_whitespace().map(str::to_string).collect()
}

fn value_by_result(result: RoundResult) -> u16 {
    match result {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Lose => 0,
    }
}

fn value_by_rps_play(rps_play: &RpsPlay) -> u16 {
    match rps_play {
        &RpsPlay::Rock => 1,
        &RpsPlay::Paper => 2,
        &RpsPlay::Scissors => 3,
    }
}

fn determine_round_result(other_play: &RpsPlay, my_play: &RpsPlay) -> RoundResult {
    if other_play == my_play {
        return RoundResult::Draw;
    }
    if (other_play == &RpsPlay::Rock && my_play == &RpsPlay::Scissors)
        || (other_play == &RpsPlay::Paper && my_play == &RpsPlay::Rock)
        || (other_play == &RpsPlay::Scissors && my_play == &RpsPlay::Paper) {
        return RoundResult::Lose;
    }
    
    RoundResult::Win
}

fn calculate_round_value(my_play: &RpsPlay, round_result: RoundResult) -> u16 {
    value_by_rps_play(my_play) + value_by_result(round_result)
}