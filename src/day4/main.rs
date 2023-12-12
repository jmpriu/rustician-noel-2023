use std::io::stdin;
use std::collections::HashMap;

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

    let mut current_card = 1;
    let mut total_cards = 0;
    let mut copies: HashMap<i32, i32> = HashMap::new();

    for line in stdin.lines().map(|l| l.unwrap()) {
        let (winning, owned) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning = extract_numbers(winning);
        let owned = extract_numbers(owned);
        let owned = owned.iter().filter(|x| winning.contains(x)).collect::<Vec<_>>();

        total_points += calculate_points(owned.len());

        let copies_of_current_card: i32 = if copies.contains_key(&current_card) { *(copies.get(&current_card).unwrap()) + 1} else { 1 };
        total_cards += copies_of_current_card;
        for x in 0..owned.len() {
            let current = current_card + x as i32 + 1;
            if copies.contains_key(&current) {
                copies.insert(current, copies.get(&current).unwrap() + copies_of_current_card);
            } else {
                copies.insert(current, copies_of_current_card);
            }
        }
        current_card = current_card + 1;
    }
    println!("Part 1{}", total_points);
    println!("Part2 {}", total_cards);
}