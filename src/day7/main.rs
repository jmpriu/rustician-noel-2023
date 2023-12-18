
use std::io::*;
use std::cmp::Ordering;
#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u128,
}
impl Hand {
    fn get_hand_value(&self) -> HandType {
        let mut cards = self.cards.clone();
        cards.sort_by(|b, a| get_card_value(*a).partial_cmp(&get_card_value(*b)).unwrap());
        
        // Five of a kind
        let a = cards[0];
        let b = cards[1];
        let c = cards[2];
        let d = cards[3];
        let e = cards[4];
        if a == b && b == c && c == d && d == e {
            return HandType::FiveOfAKind
        }
        // Four of a kind
        if a == b && b == c && c == d {
            return HandType::FourOfAKind;
        }
        if  b == c && c == d && d == e{
            return HandType::FourOfAKind;
        }
        // Full house
        if a == b && b == c && d == e{
            return HandType::FullHouse;
        }
        if a == b && c == d && d == e {
            return HandType::FullHouse;
        }
        // Three of a kind
        if a == b && b == c {
            return HandType::ThreeOfAKind;
        }
        if b ==c && c == d {
            return HandType::ThreeOfAKind;
        }
        if c == d && d == e {
            return HandType::ThreeOfAKind;
        }
        // Two pair
        if a == b && c == d {
            return HandType::TwoPair;
        }
        if a == b && d == e {
            return HandType::TwoPair;
        }
        if b == c && d == e {
            return HandType::TwoPair;
        }
        // One pair
        if a == b {
            return HandType::OnePair;
        }
        if b == c {
            return HandType::OnePair;
        }
        if c == d {
            return HandType::OnePair;
        }
        if d == e {
            return HandType::OnePair;
        }
        // High card
        return HandType::HighCard;
        
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
fn get_hand_value (hand: HandType) -> u8 {
    match hand {
        HandType::FiveOfAKind => 10,
        HandType::FourOfAKind => 9,
        HandType::FullHouse => 8,
        HandType::ThreeOfAKind => 7,
        HandType::TwoPair => 6,
        HandType::OnePair => 5,
        HandType::HighCard => 4,
    }
}
fn get_card_value(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 1
    }
}

fn main() {
    let mut hands: Vec<Hand> = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (cards, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<u128>().unwrap();
        let cards = cards.chars().collect::<Vec<_>>();
        hands.push(Hand {
            cards: cards.clone(),
            bid
        });
    }
    fn comparator(ha: &Hand, hb:&Hand) -> Ordering {
        let a = get_hand_value(ha.get_hand_value());
        let b = get_hand_value(hb.get_hand_value());
        if a == b {
            for x in 0..5 {
                if get_card_value(ha.cards[x]) != get_card_value(hb.cards[x]) {
                    return (get_card_value(hb.cards[x])).cmp(&get_card_value(ha.cards[x]));
                }
            }
        }
        return b.cmp(&a); 
    }

    hands.sort_by(comparator);
    hands.reverse();
    let mut total_points:u128 = 0;
    let mut rank = 1;

    for x in hands {
        println!("{:?} {:?} -> {} * {}", x.cards, x.get_hand_value(), x.bid, rank);
        total_points += x.bid * rank;
        rank = rank + 1;
    }
    println!("======");
    println!("Total points: {}", total_points)

}
