use std::{env, fs, vec};

#[derive(Debug, Copy, Clone)]
struct Race {
    time: u64,
    distance: u64,
}
fn main() {
    let input_path = env::var("aoc_2023_06_path").unwrap() + "/input.txt";
    let input = fs::read_to_string(input_path).unwrap();
    let races = parse_input(&input);
    let mut part1_result = 1;
    for r in races {
        let res = calculate_race(r);
        part1_result *= res.len();
    }
    println!("part1 result: {}", part1_result);
    let races_2 = parse_input_2(&input);
    let mut part2_result = 1;
    part2_result *= calculate_race(races_2[0]).len();
    println!("part2 result: {}", part2_result);
    let quadratic_result = calc_with_quadratic_eq(races_2[0]);
    println!("quadratic result: {}", quadratic_result);
}
fn calc_with_quadratic_eq(r: Race) -> u64 {
    // quadratic equation: d=ct*(t-ct) where d-distance,ct-charge time, t-race time
    // if ct^2 -t*ct+d=0
    // t = (t+-sqrt(t^2-4*d))/2
    let d = r.distance as f64;
    let t = r.time as f64;
    let discriminant = t * t - 4.0 * d;
    if discriminant <= 0.0 {
        return 0; //no solutions
    }
    let s = discriminant.sqrt();
    let start = (((t - s) / 2.0).floor() + 0.001) as u64 + 1;
    let end = (((t + s) / 2.0).ceil() + 0.001) as u64 - 1;
    if end > start {
        return end - start + 1;
    }

    return 0;
}
fn calculate_race(r: Race) -> Vec<u64> {
    let mut res = vec![];
    for i in 1..=r.time {
        let a = r.time - i;
        if a * i > r.distance {
            res.push(i);
        }
    }

    return res;
}
fn parse_input(input: &String) -> Vec<Race> {
    let mut parsed_input: Vec<Race> = vec![];
    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut word = String::new();
        for (j, c) in line.char_indices().skip(10) {
            if c.is_ascii_digit() {
                word.push(c);
                if j == line.len() - 1 {
                    if i == 0 {
                        let a = u64::from_str_radix(&word, 10).unwrap();
                        times.push(a);
                        word.clear();
                    }
                    if i == 1 {
                        let a = u64::from_str_radix(&word, 10).unwrap();
                        distances.push(a);
                        word.clear();
                    }
                }
            } else if c == ' ' && !word.is_empty() {
                if i == 0 {
                    let a = u64::from_str_radix(&word, 10).unwrap();
                    times.push(a);
                    word.clear();
                }
                if i == 1 {
                    let a = u64::from_str_radix(&word, 10).unwrap();
                    distances.push(a);
                    word.clear();
                }
            }
        }
    }
    for i in 0..times.len() {
        parsed_input.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }

    return parsed_input;
}

fn parse_input_2(input: &String) -> Vec<Race> {
    let mut parsed_input: Vec<Race> = vec![];
    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut word = String::new();
        for (j, c) in line.char_indices().skip(10) {
            if c.is_ascii_digit() {
                word.push(c);
                if j == line.len() - 1 {
                    if i == 0 {
                        let a = u64::from_str_radix(&word, 10).unwrap();
                        times.push(a);
                        word.clear();
                    }
                    if i == 1 {
                        let a = u64::from_str_radix(&word, 10).unwrap();
                        distances.push(a);
                        word.clear();
                    }
                }
            }
        }
    }
    for i in 0..times.len() {
        parsed_input.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }
    return parsed_input;
}
