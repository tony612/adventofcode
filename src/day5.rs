use std::fs;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let body = fs::read_to_string("./input/day5.txt").expect("couldn't read the input file");
    let body = body.trim();
    let mut chars: Vec<_> = body.chars().collect();
    let mut i = 0;
    loop {
        if i == chars.len() - 1 {
            break;
        }
        if chars[i].eq_ignore_ascii_case(&chars[i+1]) && chars[i] != chars[i+1] {
            chars.remove(i);
            chars.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
    println!("Part1: {}", chars.len());
}

fn part2() {
    let body = fs::read_to_string("./input/day5.txt").expect("couldn't read the input file");
    let body = body.trim();
    let chars0: Vec<_> = body.chars().collect();
    let mut min = chars0.len();
    for unit in 97..=122 {
        let unit = char::from(unit);
        let mut chars = chars0.clone();
        let mut i = 0;
        loop {
            if chars[i].eq_ignore_ascii_case(&unit) {
                chars.remove(i);
                if i > 0 {
                    i -= 1;
                }
                continue;
            }
            if i == chars.len() - 1 {
                break;
            }
            if chars[i].eq_ignore_ascii_case(&chars[i+1]) && chars[i] != chars[i+1] {
                chars.remove(i);
                chars.remove(i);
                if i > 0 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }
        let new_len = chars.len();
        if new_len < min {
            min = new_len
        }
    }
    println!("Part2: {}", min);
}
