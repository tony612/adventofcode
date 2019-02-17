use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn run() {
    let body = fs::read_to_string("./input/day6.txt").expect("couldn't read the input file");

    let mut points = vec![];

    let mut x_min = i32::max_value();
    let mut x_max: i32 = 0;
    let mut y_min = i32::max_value();
    let mut y_max: i32 = 0;

    for line in body.lines() {
        // println!("{}", line);
        let point: Vec<_> = line.split(", ").collect();
        let x: i32 = point[0].parse().unwrap();
        let y: i32 = point[1].parse().unwrap();
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
        if y < y_min {
            y_min = y;
        }
        if y > y_max {
            y_max = y;
        }
        points.push(vec![x, y]);
    }

    let x_range: Vec<_> = (x_min..=x_max).collect();
    let y_range: Vec<_> = (y_min..=y_max).collect();
    let x_range_len = x_range.len();
    let y_range_len = y_range.len();

    let mut grid = vec![];
    let mut boundary = HashSet::new();
    let mut distance_sums = vec![];
    for (x_i, x) in x_range.iter().enumerate() {
        for (y_i, y) in y_range.iter().enumerate() {
            let mut min_distance = i32::max_value();
            let mut min_distance_i = 0;
            let mut distance_sum = 0;
            for (i, p) in points.iter().enumerate() {
                let i = i as i32;
                let distance = (p[0] - x).abs() + (p[1] - y).abs();
                if distance < min_distance {
                    min_distance = distance;
                    min_distance_i = i;
                } else if distance == min_distance {
                    min_distance_i = -1;
                }
                distance_sum += distance;
            }
            grid.push(min_distance_i);
            distance_sums.push(distance_sum);
            if min_distance_i >= 0
                && (x_i == 0 || y_i == 0 || x_i == x_range_len - 1 || y_i == y_range_len - 1)
            {
                boundary.insert(min_distance_i);
            }
        }
    }
    // println!("Grid: {:?}", grid);
    // println!("Boundary: {:?}", boundary);

    let mut areas: HashMap<i32, u32> = HashMap::new();
    for id in grid.iter() {
        if *id >= 0 {
            let found = areas.get(id);
            match found {
                Some(size) => areas.insert(*id, size + 1),
                None => areas.insert(*id, 1),
            };
        }
    }
    // println!("Areas: {:?}", areas);

    let mut max_dist = u32::min_value();
    for (id, dist) in areas {
        if !boundary.contains(&id) && dist > max_dist {
            max_dist = dist;
        }
    }
    println!("Part1: {}", max_dist);

    let mut size = 0;
    for sum in distance_sums {
        if sum < 10000 {
            size += 1;
        }
    }
    println!("Part2: {}", size);
}
