use std::fs;

static PATH: &'static str = "input.txt";

fn main() {
    let mut sum = 0;
    let contents = fs::read_to_string(PATH).expect("Something went wrong reading the file");
    let mut lines = contents.split("\n").collect::<Vec<&str>>();
    lines.pop();
    let lines: Vec<Vec<&str>> = lines.chunks(3).map(|x| x.to_vec()).collect();
    for group in lines {
        for letter in group[0].chars() {
            if group[1].contains(letter) && group[2].contains(letter) {
                sum += if letter.is_lowercase() {-96} else {-64 + 26} + letter as i32;
                break;
            }
        }
    }
    println!("{}", sum);
}
