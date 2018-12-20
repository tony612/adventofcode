use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    part1();
}

fn part1() {
    let body = fs::read_to_string("./input/day4.txt").expect("couldn't read the input file");
    let line_re = Regex::new(r"\[(.+)\] (.+)").unwrap();
    let mut lines: Vec<(&str, &str)> = body.lines().map(|l| {
        let caps = line_re.captures(l).unwrap();
        (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str())
    }).collect();
    lines.sort_by(|(a, _), (b, _)| a.cmp(b));

    let info_re = Regex::new(r"Guard (\S+) begins shift").unwrap();

    let mut guards: HashMap<String, Vec<u32>> = HashMap::new();
    let mut guards_count: HashMap<&str, u32> = HashMap::new();
    let mut guard = "";
    let mut asleep = false;
    let mut begin_min = 0;
    let mut end_min;
    for (time, info) in lines {
        let (hour, min) = parse_time(time);
        match info_re.captures(info) {
            None =>
                if info == "falls asleep" {
                    asleep = true;
                    if hour == 0 {
                        begin_min = min;
                    } else {
                        begin_min = 0;
                    }
                } else if info == "wakes up" {
                    asleep = false;
                    if hour == 0 {
                        end_min = min;
                    } else {
                        end_min = 60;
                    }
                    guards = set_asleep(guards, guard, begin_min, end_min);
                    *guards_count.entry(guard).or_insert(0) += end_min - begin_min;
                },
            Some(matched) => {
                if asleep {
                    end_min = 60;
                    guards = set_asleep(guards, guard, begin_min, end_min);
                    *guards_count.entry(guard).or_insert(0) += end_min - begin_min;
                }
                guard = matched.get(1).unwrap().as_str();
            }
        }
    }
    let (max_key, total_count) = guards_count.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    let (min, count) = guards.get(&max_key.to_string()).unwrap()
        .iter().enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

    println!("guard: {}, minute: {}; total_count: {}, count: {}", max_key, min, total_count, count);
}

fn parse_time(time: &str) -> (u32, u32) {
    let hm: Vec<&str> = time.split(' ').collect();
    let hm: Vec<&str> = hm[1].split(':').collect();
    (hm[0].parse::<u32>().unwrap(), hm[1].parse::<u32>().unwrap())
}

fn set_asleep(mut guards: HashMap<String, Vec<u32>>, id: &str, begin: u32, end: u32) -> HashMap<String, Vec<u32>> {
    let guard = guards.entry(id.to_string()).or_insert(vec![0; 60]);
    for i in begin..end {
        guard[i as usize] += 1;
    }
    guards
}
