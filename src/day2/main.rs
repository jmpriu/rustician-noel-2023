use std::io::stdin;
const TOTAL_REDS: i32 = 12;
const TOTAL_GREEN: i32 = 13;
const TOTAL_BLUES: i32 = 14;

fn main() {
    let stdin = stdin();
    let mut sum = 0;
    let mut sum_power = 0;
    for games in stdin.lines().map(|l| l.unwrap()) {
        let (game_id, game_results) = games.split_once(": ").unwrap();
        let (_, game_id) = game_id.split_once(" ").unwrap();
        let game_id = game_id.parse::<i32>().unwrap();
        let mut blues = 0;
        let mut reds = 0;
        let mut green = 0;
        for round in game_results.split("; ") {
            for cube in round.split(", ") {
                let (amount, color) = cube.split_once(" ").unwrap();
                let amount = amount.parse().unwrap();
                match color {
                    "blue" => blues = if amount > blues { amount } else { blues },
                    "red" => reds = if amount > reds { amount } else { reds },
                    "green" => green = if amount > green { amount } else { green },
                    _ => panic!("Invalid color"),
                }

            }
        }
        println!("{game_id}: {}", blues * reds * green);
        sum_power += blues * reds * green;
        if blues <= TOTAL_BLUES && reds <= TOTAL_REDS && green <= TOTAL_GREEN {
            sum += game_id;
        }
    }
    println!("Sum: {sum}; Power: {sum_power}");
}