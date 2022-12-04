use std::fs::File;
use std::io::{prelude::*, BufReader};


enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    fn new(hand: &str) -> Hand {

        match hand {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!()
        }
    }

    fn points(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }

    fn wins_over(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper
        }
    }

    fn looses_over(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock
        }
    }

    fn for_outcome(self, outcome: &Outcome) -> Hand {
        match outcome {
            Outcome::Win => self.looses_over(),
            Outcome::Lose => self.wins_over(),
            Outcome::Draw => self
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw
}

impl Outcome {
    
    fn new(outcome: &str) -> Outcome {
        match outcome {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!()
        }
    }

    fn points(&self) -> i32 {
        match self {
            Outcome::Draw => 3,
            Outcome::Lose => 0,
            Outcome::Win => 6
        }
    }
}

fn eval_match(play: (Hand, Hand)) -> i32 {

    match play {
        (Hand::Rock, Hand::Paper) => 6,
        (Hand::Paper, Hand::Scissors) => 6,
        (Hand::Scissors, Hand::Rock) => 6,
        (Hand::Rock, Hand::Scissors) => 0,
        (Hand::Paper, Hand::Rock) => 0,
        (Hand::Scissors, Hand::Paper) => 0,
        (Hand::Rock, Hand::Rock) => 3,
        (Hand::Paper, Hand::Paper) => 3,
        (Hand::Scissors, Hand::Scissors) => 3
    }
}

fn parse_row(row: &str) -> (Hand, Hand) {
    let hands: Vec<&str> = row.split(' ').map(|s| s).collect();
    (Hand::new(hands[0]), Hand::new(hands[1]))
}

fn parse_row_second(row: &str) -> (Hand, Outcome) {
    let hands: Vec<&str> = row.split(' ').map(|s| s).collect();
    (Hand::new(hands[0]), Outcome::new(hands[1]))
}

fn first() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut total_score: i32 = 0;

    for line in reader.lines() {

        let round = line.unwrap();
        
        let hands = parse_row(&round);
        
        let (_, player_hand) = &hands;
        
        total_score += player_hand.points();
        total_score += eval_match(hands);
    }

    println!("Total score: {}", total_score);
}

fn second() {

    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut total_score: i32 = 0;

    for line in reader.lines() {

        let round = line.unwrap();
        
        let (hand, outcome) = parse_row_second(&round);
        
        let player_hand = hand.for_outcome(&outcome);
        
        total_score += player_hand.points();
        total_score += outcome.points();
    }

    println!("Total score: {}", total_score);
}

fn main() {
    first();
    second();
}
