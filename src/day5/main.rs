use std::io::*;

fn main() {
    let stdin = stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let mut seeds: Vec<u64> = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut changes: Vec<bool> = vec![false; seeds.len()];
    for i in 0..lines.len() {
        let line = lines[i].clone();
        if line.len() > 0 && line.chars().nth(0).unwrap().is_numeric() {
            let ranges = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
            let destination = ranges[0];
            let origin = ranges[1];
            let range = ranges[2];
            for x in 0..seeds.len() {
                if !changes[x] && seeds[x] >= origin && seeds[x] <= origin+range {
                    seeds[x] = seeds[x] + destination - origin;
                    changes[x] = true;
                }
            }
        }else {
            changes = vec![false; seeds.len()];
        }
    }
    let mut min = seeds[0];
    for x in seeds {
        if min > x {
            min = x;
        }
    }
    println!("Min: {}", min);
}
