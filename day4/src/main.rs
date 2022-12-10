use std::fs;
use std::ops::RangeInclusive;

static PATH: &'static str = "input.txt";

fn main() {
    let mut count: u32 = 0;
    let contents = fs::read_to_string(PATH).expect("Something went wrong reading the file");
    let mut values: Vec<&str> = contents.split("\n").collect();
    values.pop();
    let values: Vec<Vec<&str>> = values.iter().map(|x| x.split(",").collect()).collect();
    let values: Vec<Vec<RangeInclusive<u32>>>= values.iter().map(|x| {
        x.iter().map(|y| {
        let y = y.split("-").collect::<Vec<&str>>();
        y[0].parse::<u32>().unwrap()..=y[1].parse::<u32>().unwrap()
        }).collect()
    }).collect();
    for group in values {
        if group[0].contains(group[1].start()) && group[0].contains(group[1].end()) || group[1].contains(group[0].start()) && group[1].contains(group[0].end()) {
            count += 1;
        }
    }
    println!("{}", count);
}
