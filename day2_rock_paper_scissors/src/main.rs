use std::fmt;

pub mod data;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    let optimal_round_scores = read_to_round_scores_as_optimal();
    println!("The Sum of All Optimal Rounds is: {}", sum_scores(optimal_round_scores));

    let strategic_round_scores = read_to_round_scores_as_strategy();
    println!("The Sum of All Strategic Rounds is: {}", sum_scores(strategic_round_scores));
}

fn sum_scores(round_scores: Vec<u16>) -> u16 {  
    round_scores.iter().sum()
}

fn read_to_round_scores_as_optimal() -> Vec::<u16>{
    let mut round_score = Vec::<u16>::new();
    if let Ok(lines) = data::read_data_from_file() {
        for line in lines {
            if let Ok(round_plays) = line {
                if !round_plays.is_empty() {

                    let plays = split_line_into_codes(round_plays);
                    let other_play = play_from_code(plays[0].as_str()).clone();
                    let my_play = play_from_code(plays[1].as_str()).clone();
                    let result = determine_round_result(other_play, my_play.clone());
                    let score = calculate_round_value(my_play.clone(), result);

                    round_score.push(score);
                }
            }
        }
    }
    // Return
    round_score
}

fn read_to_round_scores_as_strategy() -> Vec::<u16>{
    let mut round_score = Vec::<u16>::new();
    if let Ok(lines) = data::read_data_from_file() {
        for line in lines {
            if let Ok(round_plays) = line {
                if !round_plays.is_empty() {
                    let codes = split_line_into_codes(round_plays);
                    let other_play = play_from_code(codes[0].as_str());
                    let result: RoundResult = result_from_code(codes[1].as_str());
                    let my_play: RpsPlay = determine_my_play_by_result(other_play, result.clone());
                    let score = calculate_round_value(my_play.clone(), result.clone());
                    // println!("Other Play: {other_play}\tMy Play: {my_play}\t\tScore: {score}");
                    round_score.push(score);
                }
            }
        }
    }
    // Return
    round_score
}

fn split_line_into_codes(line: String) -> Vec<String> {
    line.split_whitespace().map(str::to_string).collect()
}

fn play_from_code(code: &str) -> RpsPlay {
    match code {
        "A" => RpsPlay::Rock,
        "B" => RpsPlay::Paper,
        "C" => RpsPlay::Scissors,
        "X" => RpsPlay::Rock,
        "Y" => RpsPlay::Paper,
        "Z" => RpsPlay::Scissors,
        &_ => RpsPlay::Scissors,
    }
}

fn result_from_code(code: &str) -> RoundResult {
    match code {
        "X" => RoundResult::Lose,
        "Y" => RoundResult::Draw,
        "Z" => RoundResult::Win,
        &_ => RoundResult::Lose,
    }
}

fn value_by_result(result: RoundResult) -> u16 {
    match result {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Lose => 0,
    }
}

fn value_by_rps_play(rps_play: RpsPlay) -> u16 {
    match rps_play {
        RpsPlay::Rock => 1,
        RpsPlay::Paper => 2,
        RpsPlay::Scissors => 3,
    }
}

fn my_losing_play_by_other_play(other_play: RpsPlay) -> RpsPlay {
    match other_play {
        RpsPlay::Rock => RpsPlay::Scissors,
        RpsPlay::Paper => RpsPlay::Rock,
        RpsPlay::Scissors => RpsPlay::Paper,
    }
}

fn my_winning_play_by_other_play(other_play: RpsPlay) -> RpsPlay {
    match other_play {
        RpsPlay::Scissors => RpsPlay::Rock,
        RpsPlay::Rock => RpsPlay::Paper,
        RpsPlay::Paper => RpsPlay::Scissors,
    }
}

fn determine_my_play_by_result(other_play: RpsPlay, expected_result: RoundResult) -> RpsPlay {
    if expected_result == RoundResult::Draw {
        return other_play;
    }

    if expected_result == RoundResult::Lose{
        return my_losing_play_by_other_play(other_play);
    }

    my_winning_play_by_other_play(other_play)
}

fn determine_round_result(other_play: RpsPlay, my_play: RpsPlay) -> RoundResult {
    if other_play == my_play {
        return RoundResult::Draw;
    }
    if (other_play == RpsPlay::Rock && my_play == RpsPlay::Scissors)
        || (other_play == RpsPlay::Paper && my_play == RpsPlay::Rock)
        || (other_play == RpsPlay::Scissors && my_play == RpsPlay::Paper) {
        return RoundResult::Lose;
    }
    
    RoundResult::Win
}

fn calculate_round_value(my_play: RpsPlay, round_result: RoundResult) -> u16 {
    value_by_rps_play(my_play) + value_by_result(round_result)
}