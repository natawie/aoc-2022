use std::fs;

static PATH: &'static str = "input.txt";

fn when_is_unique(input: &Vec<&str>, length: usize) -> usize {
    let mut count: usize = 0;
    loop {
        // this is the cleanest way i can think of
        let v = input[count..count+length].to_vec();
        let mut v2 = v.clone();
        v2.sort();
        v2.dedup();
        if v.len() == v2.len() {
            break;
        }
        count += 1
    }
    return count + length;
}

fn main() {
    let contents = fs::read_to_string(PATH).expect("Something went wrong reading the file");
    let mut contents = contents.split("").collect::<Vec<&str>>();
    contents.remove(0);
    contents.pop();
    println!("part 1: {}", when_is_unique(&contents, 4));
    println!("part 2: {}", when_is_unique(&contents, 14));
}
