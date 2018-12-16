use std::fs;

pub fn run() {
    let body = fs::read_to_string("./input/day2-1.txt").expect("couldn't read the input file");

    let lines = body.lines();
    let mut two = 0;
    let mut three = 0;
    for line in lines {
        let mut chars: Vec<char> = line.chars().collect();
        chars.sort();
        // check at last
        chars.push('\0');
        let mut curr_char = '\0';
        let mut curr_count = 0;
        let mut has_two = false;
        let mut has_three = false;
        for b in chars.iter() {
            if *b == curr_char {
                curr_count += 1;
            } else {
                if !has_two && curr_count == 2 {
                    has_two = true;
                    two += 1;
                }
                if !has_three && curr_count == 3 {
                    has_three = true;
                    three += 1;
                }
                curr_count = 1;
                curr_char = *b;
            }
        }
    }
    println!("Result is {} * {} = {}", two, three, two * three);
}
