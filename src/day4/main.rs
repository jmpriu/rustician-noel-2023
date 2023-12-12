use std::io::stdin;

fn extract_numbers (numbers: &str) -> Vec<i32> {
    numbers.split(" ").map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()
}

fn calculate_points (total: usize) -> i32 {
    if total == 0 {
        return 0;
    }
    return 1 << total - 1;
}
fn main() {
    let stdin = stdin();
    let mut total_points = 0;
    for line in stdin.lines().map(|l| l.unwrap()) {
        let (winning, owned) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning = extract_numbers(winning);
        let owned = extract_numbers(owned);
        let owned = owned.iter().filter(|x| winning.contains(x)).collect::<Vec<_>>();
        total_points += calculate_points(owned.len());
    }
    println!("{}", total_points);
}