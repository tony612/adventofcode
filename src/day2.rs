use std::fs;

pub fn run() {
    part1();
    part2();
}

fn part1() {
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

fn part2() {
    let body = fs::read_to_string("./input/day2-1.txt").expect("couldn't read the input file");
    let lines: Vec<&str> = body.lines().collect();
    let len = lines.len();
    let mut diff = 100;
    let mut common: Vec<char> = vec!();

    for i in 0..len {
        for j in (i+1)..len {
            match id_diff(lines[i], lines[j]) {
                (curr_diff, curr_common) =>
                    if curr_diff < diff {
                        diff = curr_diff;
                        common = curr_common;
                    }
            }
        }
    }
    let common = common.iter().collect::<String>();
    println!("Result is {} {:?}", diff, common);
}

fn id_diff(str1: &str, str2: &str) -> (u32, Vec<char>) {
    let chars1 = str1.chars();
    let chars2 = str2.chars();
    let mut diff: u32 = 0;
    let mut common: Vec<char> = vec!();
    for (c1, c2) in chars1.zip(chars2) {
        if c1 == c2 {
            common.push(c1);
        } else {
            diff += 1;
        }
    }
    (diff, common)
}
