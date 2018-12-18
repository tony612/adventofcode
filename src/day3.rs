use std::fs;
use std::cmp;
use std::collections::HashMap;

struct Rect {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}

pub fn run() {
    let body = fs::read_to_string("./input/day3.txt").expect("couldn't read the input file");

    let lines: Vec<&str> = body.lines().collect();
    let len = lines.len();
    let mut rects: Vec<Rect> = vec!();

    for i in 0..len {
        let rect = parse_rect(lines[i]);
        rects.push(rect);
    }

    let mut faric = HashMap::new();
    let mut overlap = HashMap::new();

    for i in 0..len {
        let rect_i = &rects[i];
        for j in (i+1)..len {
            let rect_j = &rects[j];
            let left = cmp::max(rect_i.left, rect_j.left);
            let right = cmp::min(rect_i.right, rect_j.right);
            let top = cmp::max(rect_i.top, rect_j.top);
            let bottom = cmp::min(rect_i.bottom, rect_j.bottom);
            if left <= right && top <= bottom {
                for x in left..=right {
                    for y in top..=bottom {
                        faric.insert(format!("{},{}", x, y), true);
                    }
                }
                // part2
                overlap.insert(i, true);
                overlap.insert(j, true);
            }
        }
    }
    println!("Part1 result: {}", faric.len());

    // part2
    for i in 0..len {
        if let None = overlap.get(&i) {
            println!("Part2 result: {}", lines[i]);
            break;
        }
    }
}

fn parse_rect(id: &str) -> Rect {
    let parts: Vec<&str> = id.splitn(4, ' ').collect();
    let lt: Vec<&str> = parts[2].splitn(3, |c| c == ',' || c == ':').collect();
    let wh: Vec<&str> = parts[3].splitn(2, 'x').collect();
    let left = lt[0].parse::<u32>().unwrap();
    let top = lt[1].parse::<u32>().unwrap();
    let rect = Rect {
        left: left,
        top: top,
        right: left + wh[0].parse::<u32>().unwrap() - 1,
        bottom: top + wh[1].parse::<u32>().unwrap() - 1,
    };
    rect
}
