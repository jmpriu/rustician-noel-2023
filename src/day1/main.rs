use std::io::stdin;

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let stdin = stdin();
    let mut total = 0;
    for mut line in stdin.lines().map(|l| l.unwrap()) {
        while !NUMBERS.iter().any(|x| line.starts_with(x)) && !line.chars().next().unwrap().is_numeric() {
            line.remove(0);
        }
        while !NUMBERS.iter().any(|x| line.ends_with(x)) && !line.chars().last().unwrap().is_numeric() {
            line.pop();
        }
        let mut decimal = 0;
        let mut units = 0;
        for (index, num) in NUMBERS.iter().enumerate() {
            if line.starts_with(num) {
                decimal = index + 1;
            }
            if line.ends_with(num) {
                units = index + 1;
            }
        }
        if line.chars().next().unwrap().is_numeric() {
            decimal = line.chars().next().unwrap().to_digit(10).unwrap() as usize;
        }
        if line.chars().last().unwrap().is_numeric() {
            units = line.chars().last().unwrap().to_digit(10).unwrap() as usize;
        }
        total += decimal * 10 + units;
    }
    println!("{}", total);
}