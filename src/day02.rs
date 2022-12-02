use std::fs;

#[derive(Debug, Eq, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

fn translate_1st_play(play: &str) -> Play {
    match play {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => panic!("Error!"),
    }
}

fn translate_2nd_play(play: &str) -> Play {
    match play {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => panic!("Error!"),
    }
}

#[derive(Debug)]
enum Winner {
    Draw,
    Player1,
    Player2,
}

fn determine_winner(play_1: &Play, play_2: &Play) -> Winner {
    match play_1 {
        Play::Rock => match play_2 {
            Play::Rock => Winner::Draw,
            Play::Paper => Winner::Player2,
            Play::Scissors => Winner::Player1,
        },
        Play::Paper => match play_2 {
            Play::Rock => Winner::Player1,
            Play::Paper => Winner::Draw,
            Play::Scissors => Winner::Player2,
        },
        Play::Scissors => match play_2 {
            Play::Rock => Winner::Player2,
            Play::Paper => Winner::Player1,
            Play::Scissors => Winner::Draw,
        },
    }
}

fn get_score_for_play(play: &Play) -> i32 {
    match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    }
}

fn get_score_for_win(winner: &Winner) -> i32 {
    match winner {
        Winner::Player1 => 0,
        Winner::Draw => 3,
        Winner::Player2 => 6,
    }
}

fn part_2_translate_2nd_play(play: &str) -> Winner {
    match play {
        "X" => Winner::Player1,
        "Y" => Winner::Draw,
        "Z" => Winner::Player2,
        _ => panic!("Error!"),
    }
}

fn part_2_find_matching_play(play_1: &Play, outcome: &Winner) -> Play {
    match play_1 {
        Play::Rock => match outcome {
            Winner::Player1 => Play::Scissors,
            Winner::Draw => Play::Rock,
            Winner::Player2 => Play::Paper,
        },
        Play::Paper => match outcome {
            Winner::Player1 => Play::Rock,
            Winner::Draw => Play::Paper,
            Winner::Player2 => Play::Scissors,
        },
        Play::Scissors => match outcome {
            Winner::Player1 => Play::Paper,
            Winner::Draw => Play::Scissors,
            Winner::Player2 => Play::Rock,
        },
    }
}

fn part_1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let plays = line.split(" ").collect::<Vec<&str>>();

        println!("{}, {}", plays[0], plays[1]);

        let play_1 = translate_1st_play(plays[0]);
        let play_2 = translate_2nd_play(plays[1]);

        println!("{:?}, {:?}", play_1, play_2);

        let winner = determine_winner(&play_1, &play_2);

        println!("{:?}", winner);
        println!();

        sum += get_score_for_play(&play_2);
        sum += get_score_for_win(&winner);
    }

    println!("{}", sum);
}

fn part_2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let plays = line.split(" ").collect::<Vec<&str>>();

        let play_1 = translate_1st_play(plays[0]);
        let winner = part_2_translate_2nd_play(plays[1]);

        let play_2 = part_2_find_matching_play(&play_1, &winner);

        sum += get_score_for_play(&play_2);
        sum += get_score_for_win(&winner);
    }

    println!("{}", sum);
}

fn main() {
    let input = fs::read_to_string("./input_day02.txt").expect("Couldn't find file.");

    println!("{}", input.lines().collect::<Vec<&str>>().len());

    part_1(&input);
    part_2(&input);
}
