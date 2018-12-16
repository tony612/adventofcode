use std::fs;
use std::collections::HashMap;

pub fn run() {
  part1();
  part2();
}

fn part1() {
    let body = fs::read_to_string("./input/day1-1.txt").expect("couldn't read the input file");

    let mut result = 0;
    for line in body.lines() {
        // println!("{}", line);
        let number: i32 = line.parse().unwrap();
        result += number;
    }
    println!("Result is: {}", result);
}

fn part2() {
    let body = fs::read_to_string("./input/day1-1.txt").expect("couldn't read the input file");

    let mut result = 0;
    let mut sum = 0;
    let mut sums: HashMap<i32, bool> = HashMap::new();
    loop {
        for line in body.lines() {
            // println!("{}", line);
            let number: i32 = line.parse().unwrap();
            sum += number;
            let found = sums.get(&sum);
            match found {
                Some(_) => {
                    result = sum;
                    break;
                },
                None =>
                    sums.insert(sum, true),
            };
        }
        if result != 0 || sum == 0 {
            break;
        }
    }
    println!("Result is: {}", result);
}
