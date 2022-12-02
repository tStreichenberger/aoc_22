use std::collections::HashMap;

use crate::{AOCResult,CustomError};

pub fn solution_1(tournament: String) -> AOCResult<u32> {
    let mut total_points = 0;
    for game in tournament.split("\n") {
        total_points += calc_points_1(game)?;
    }
    Ok(total_points)
}

fn calc_points_1(game: &str) -> AOCResult<u32> {
    let (lhs, rhs) = game.split_at(1);
    let elf_move = RPS::parse(lhs)?;
    let my_move = RPS::parse(rhs)?;
    let outcome = Outcome::from_match(elf_move, my_move);
    Ok(outcome.points() + my_move.points())
}



pub fn solution_2(tournament: String) -> AOCResult<u32> {
    let mut total_points = 0;
    for game in tournament.split("\n") {
        total_points += calc_points_2(game)?;
    }
    Ok(total_points)
}

fn calc_points_2(game: &str) -> AOCResult<u32> {
    let (lhs, rhs) = game.split_at(1);
    let elf_move = RPS::parse(lhs)?;
    let outcome = Outcome::parse(rhs)?;
    let my_move = RPS::from_outcome(outcome, elf_move);
    Ok(outcome.points() + my_move.points())
}


// Three choices to play in Rock Paper Scissors
#[derive(Copy, Clone, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn points(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }

    fn beats(self) -> Self {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS:: Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn draws(self) -> Self {self}

    fn loses_to(self) -> Self {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS:: Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn parse(letter: &str) -> AOCResult<Self> {
        let letter = letter.trim();
        match letter {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(CustomError(format!("Tried to parse {letter}, something other than A|B|C|X|Y|Z")).into())
        }
    }

    fn from_outcome(outcome: Outcome, lhs: RPS) -> Self {
        match outcome {
            Outcome::Win => lhs.loses_to(),
            Outcome::Draw => lhs.draws(),
            Outcome::Lose => lhs.beats()
        }
    }
}

// represents the outcome for the rhs of the match
#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose
}

impl Outcome {
    fn points(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0
        }
    }

    fn parse(letter: &str) -> AOCResult<Self> {
        let letter = letter.trim();
        match letter {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(CustomError(format!("Tried to parse {letter}, something other than X|Y|Z")).into())
        }
    }

    fn from_match(lhs: RPS, rhs: RPS) -> Self {
        if rhs.beats() == lhs {Outcome::Win}
        else if rhs.draws() == lhs {Outcome::Draw}
        else {Outcome::Lose}
    }
}