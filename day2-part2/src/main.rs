use std::fs;

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome { 
    Win,
    Lose,
    Tie,
}

static ROCK_VALUE: u32 = 1;
static PAPER_VALUE: u32 = 2;
static SCISSORS_VALUE: u32 = 3;
static WIN_VALUE: u32 = 6;
static LOSE_VALUE: u32 = 0;
static TIE_VALUE: u32 = 3;

static PATH: &'static str = "input.txt";

fn main() {
    let mut score: u32 = 0;
    let contents = fs::read_to_string(PATH).expect("Something went wrong reading the file");
    let mut groups: Vec<&str> = contents.split("\n").collect();
    groups.pop();
    let groups: Vec<[&str; 2]> = groups.iter().map(|x| x.split(" ").collect::<Vec<&str>>().try_into().unwrap()).collect();
    let mut groups_enumed: Vec<(Shape, Outcome)> = vec!();
    for group in groups.iter() {
        let shape = match group[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid shape"),
        };
        let outcome = match group[1] {
            "X" => Outcome::Lose,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome"),
        };
        groups_enumed.push((shape, outcome));
    }

    for group in groups_enumed.iter() {
        let (shape, outcome) = group;
        let outcome_value = match outcome {
            Outcome::Win => WIN_VALUE,
            Outcome::Lose => LOSE_VALUE,
            Outcome::Tie => TIE_VALUE,
        };
        let shape_value = match outcome {
            Outcome::Win => match shape {
                Shape::Rock => PAPER_VALUE,
                Shape::Paper => SCISSORS_VALUE,
                Shape::Scissors => ROCK_VALUE,
            },
            Outcome::Lose => match shape {
                Shape::Rock => SCISSORS_VALUE,
                Shape::Paper => ROCK_VALUE,
                Shape::Scissors => PAPER_VALUE,
            },
            Outcome::Tie => match shape {
                Shape::Rock => ROCK_VALUE,
                Shape::Paper => PAPER_VALUE,
                Shape::Scissors => SCISSORS_VALUE,
            },
        };
        score += shape_value + outcome_value;
    }
    println!("{}", score);
}
