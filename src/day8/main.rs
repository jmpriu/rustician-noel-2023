use std::io::*;
struct Path {
    initial: String,
    left: String,
    right: String,
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
    let mut steps = 0;
    let mut current = paths.iter().find(|p| p.initial == "AAA").unwrap();
    'outer: loop {
        for x in instructions.clone() {
            print!("{}: {} {x} -> ", steps, current.initial);
            current = if x == 'L' {
                paths.iter().find(|p| p.initial == current.left).unwrap()
            } else {
                paths.iter().find(|p| p.initial == current.right).unwrap()
            };
            steps = steps + 1;
            print!("{}\n", current.initial);
        }
        if current.initial == "ZZZ" {
            break 'outer;
        }
    }
    println!("{steps}")
}
