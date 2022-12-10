use std::fs;

static PATH: &'static str = "input.txt";

fn main() {
    let contents = fs::read_to_string(PATH).expect("Something went wrong reading the file");
    let contents = contents.split("\n").collect::<Vec<&str>>();
    let (table, instructions) = contents.split_at(contents.clone().into_iter().position(|x| x == "").unwrap());
    // parsing table
    // // convert Vec<&str> to Vec<Vec<char>>
    let table = table.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    // // create a variable to store parsed values
    let mut stacks: Vec<Vec<char>> = Vec::new();
    // // iterate over each row - i know this is extremely ugly, but it works sooooo yeah
    for i in 1..(*table.clone().last().unwrap().into_iter().rev().collect::<Vec<&char>>()[1] as u8 - 48)+1 {
    // // get position of column number which is how elements are alligned in the table
        let pos = table.last().unwrap().iter().position(|x| *x as u8 == i+48).unwrap();
        let mut stack: Vec<char> = Vec::new();
    //  // this is also very ugly
        for line in table.clone().into_iter().rev().skip(1).rev() {
    // // if the char at pos isn't a space, push it to the stack
            if line[pos] != ' ' {
                stack.push(line[pos]);
            }
        }
    // // each stack is reversed, so we don't have to reverse each stack just to pop from the top
        stack.reverse();
        stacks.push(stack);
    }
    // parsing instructions
    for instruction in &instructions[1..] {
        if instruction == &"" {
            break;
        }
    // // split instruction into parts
        let instruction = instruction.split(" ").collect::<Vec<&str>>();
        // // since instructions are always in the same order, we can just hardcode it 
        // // [1] - quantity
        // // [3] - source stack
        // // [5] - destination stack
        let quantity = instruction[1].parse::<usize>().unwrap();
        let source = instruction[3].parse::<usize>().unwrap() - 1;
        let destination = instruction[5].parse::<usize>().unwrap() - 1;
        for _ in 0..quantity {
        // // pop from source stack and push to destination stack
            let item = stacks[source].pop().unwrap();
            // println!("Moving {} from stack {} to stack {}", item, source, destination);
            stacks[destination].push(item);
        }
    }
    // printing
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}
