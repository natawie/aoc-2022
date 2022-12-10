use std::fs;

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
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
    let groups : Vec<[Shape; 2]> = groups.iter().map(|x| x.iter().map(|y| match y {
        &"A" | &"X" => Shape::Rock,
        &"B" | &"Y" => Shape::Paper,
        &"C" | &"Z" => Shape::Scissors,
        _ => panic!("Invalid shape"),
    }).collect::<Vec<Shape>>().try_into().unwrap()).collect();
    for group in groups {
        match group {
            [Shape::Rock, Shape::Paper] => score += PAPER_VALUE + WIN_VALUE,
            [Shape::Rock, Shape::Rock] => score += ROCK_VALUE + TIE_VALUE,
            [Shape::Rock, Shape::Scissors] => score += SCISSORS_VALUE + LOSE_VALUE,
            [Shape::Paper, Shape::Rock] => score += ROCK_VALUE + LOSE_VALUE,
            [Shape::Paper, Shape::Paper] => score += PAPER_VALUE + TIE_VALUE,
            [Shape::Paper, Shape::Scissors] => score += SCISSORS_VALUE + WIN_VALUE,
            [Shape::Scissors, Shape::Rock] => score += ROCK_VALUE + WIN_VALUE,
            [Shape::Scissors, Shape::Paper] => score += PAPER_VALUE + LOSE_VALUE,
            [Shape::Scissors, Shape::Scissors] => score += SCISSORS_VALUE + TIE_VALUE,
        }
    }
    println!("{}", score);
}
