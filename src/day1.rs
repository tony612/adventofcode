use std::fs;

pub fn run() {
    let body = fs::read_to_string("./input/day1-1.txt").expect("couldn't read the input file");

    let lines = body.lines();
    let mut result = 0;
    for line in lines {
      // println!("{}", line);
      let number: i32 = line.parse().unwrap();
      result += number;
    }
    println!("Result is: {}", result);
}
