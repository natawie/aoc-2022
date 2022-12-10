use std::fs;

static PATH: &'static str = "input.txt";

fn main() {
    let mut sum = 0;
    let contents = fs::read_to_string(PATH).expect("Something went wrong reading the file");
    let mut lines = contents.split("\n").collect::<Vec<&str>>();
    lines.pop();
    let lines = lines.iter().map(|x| x.split_at(x.len() / 2)).collect::<Vec<(&str, &str)>>();
    for (first, second) in lines {
        for letter in first.chars() {
            if second.contains(letter) {
                sum += if letter.is_lowercase() {-96} else {-64 + 26} + letter as i32;
                break;
            }
        }
    }
    println!("{}", sum);
}
