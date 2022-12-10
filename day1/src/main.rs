use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let groups: Vec<Vec<&str>> = contents.split("\n\n").map(|x| x.split("\n").collect()).collect();
    let mut groups: Vec<Vec<u32>> = groups.iter().map(|x| x.iter().map(|y| y.parse().unwrap_or(0)).collect()).collect();
    let mut groups: Vec<u32> = groups.iter().map(|x| x.iter().sum()).collect();
    groups.sort();
    println!("{:?}", groups);
}
