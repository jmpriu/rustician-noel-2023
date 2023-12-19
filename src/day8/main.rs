use std::io::*;
struct Path {
    initial: String,
    left: String,
    right: String,
}

fn lcm(first: u128, second: u128) -> u128 {
    first * second / gcd(first, second)
}

fn gcd(first: u128, second: u128) -> u128 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
fn main() {
    let mut instructions: Vec<char> = Vec::new();
    let mut paths: Vec<Path> = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        if instructions.len() == 0 {
            instructions = line.chars().collect::<Vec<_>>();
            continue;
        }
        if line == "" {
            continue;
        }
        let (initial, directions) = line.split_once(" = ").unwrap();
        let (left, right) = directions.split_once(", ").unwrap();
        let left = &left[1..];
        let right = &right[..right.len() - 1];
        paths.push(Path {
            initial: initial.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        });
    }
    let initial_nodes = paths
        .iter()
        .filter(|p| p.initial.ends_with("A"))
        .collect::<Vec<_>>();
    let mut total_iterations = Vec::new();
    for mut current in initial_nodes {
        let mut iterations: u32 = 0;
        loop {
            for x in instructions.clone() {
                current = if x == 'L' {
                    paths.iter().find(|p| p.initial == current.left).unwrap()
                } else {
                    paths.iter().find(|p| p.initial == current.right).unwrap()
                };
            }
            iterations = iterations + 1;
            if current.initial.ends_with('Z') {
                break;
            }
        }
        total_iterations.push(iterations);
    }
    let mut lcm_result = 1;
    for x in total_iterations.clone() {
        lcm_result = lcm(lcm_result, x as u128);
    }
    println!("lcm of {:?} * {} = {} * {} = {}", total_iterations, instructions.len(), lcm_result, instructions.len(), {lcm_result * instructions.len() as u128});
}
