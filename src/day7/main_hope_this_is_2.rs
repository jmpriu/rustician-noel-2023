
use std::io::*;
use std::cmp::Ordering;
#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u128,
}
impl Hand {
    fn get_hand_value(&self) -> HandType {
        // Five of a kind
        let a = self.cards[0];
        let b = self.cards[1];
        let c = self.cards[2];
        let d = self.cards[3];
        let e = self.cards[4];
        if a == b && b == c && c == d && d == e {
            return HandType::FiveOfAKind(a)
        }
        // Four of a kind
        if a == b && b == c && c == d {
            return HandType::FourOfAKind(a, e);
        }
        if  b == c && c == d && d == e{
            return HandType::FourOfAKind(b, a);
        }
        // Full house
        if a == b && b == c && d == e{
            return HandType::FullHouse(a, d);
        }
        if a == b && c == d && d == e {
            return HandType::FullHouse(c, a);
        }
        // Three of a kind
        if a == b && b == c {
            return HandType::ThreeOfAKind(a, d, e);
        }
        if b ==c && c == d {
            return HandType::ThreeOfAKind(b,a,e);
        }
        if c == d && d == e {
            return HandType::ThreeOfAKind(c,a, b);
        }
        // Two pair
        if a == b && c == d {
            return HandType::TwoPair(a, c, e);
        }
        if a == b && d == e {
            return HandType::TwoPair(a, d, c);
        }
        if b == c && d == e {
            return HandType::TwoPair(b, d, a);
        }
        // One pair
        if a == b {
            return HandType::OnePair(a, c, d, e);
        }
        if b == c {
            return HandType::OnePair(b, a, d, e);
        }
        if c == d {
            return HandType::OnePair(c, a, b, e);
        }
        if d == e {
            return HandType::OnePair(d, a, b , c);
        }
        // High card
        return HandType::HighCard(a, b, c, d, e);
        
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind(char),
    FourOfAKind(char, char),
    FullHouse(char, char),
    ThreeOfAKind(char, char, char),
    TwoPair(char, char, char),
    OnePair(char, char, char, char),
    HighCard(char, char, char, char, char),
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
        let mut cards = cards.chars().collect::<Vec<_>>();
        cards.sort_by(|b, a| get_card_value(*a).partial_cmp(&get_card_value(*b)).unwrap());
        hands.push(Hand {
            cards: cards.clone(),
            bid
        });
    }
    fn comparator(a: &Hand, b:&Hand) -> Ordering {
        let a = a.get_hand_value();
        let b = b.get_hand_value();
        match a {
            HandType::FiveOfAKind(x) => {
                if let HandType::FiveOfAKind(y) = b {
                    return (get_card_value(y)).cmp(&get_card_value(x));
                }
                return Ordering::Less;

            },
            HandType::FourOfAKind(x1,x2) => {
                if let HandType::FiveOfAKind(_)=b {
                    return Ordering::Greater;
                }
                if let HandType::FourOfAKind(y1, y2) = b {
                    if x1 == y1 {
                        return (get_card_value(y2)).cmp(&get_card_value(x2));
                    }
                    return (get_card_value(y1)).cmp(&get_card_value(x1));
                }
                return Ordering::Less;
            },
            HandType::FullHouse(x1, y1) => {
                if let HandType::FiveOfAKind(_)=b {
                    return Ordering::Greater;
                }
                if let HandType::FourOfAKind(_, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::FullHouse(x2, y2) = b {
                    if x1 == x2 {
                        return (get_card_value(y2)).cmp(&get_card_value(y1));
                    } else {
                        return (get_card_value(x2)).cmp(&get_card_value(x1));
                    }
                }
                return Ordering::Less;
            },
            HandType::ThreeOfAKind(x1, x2, x3) => {
                if let HandType::FiveOfAKind(_)=b {
                    return Ordering::Greater;
                }
                if let HandType::FourOfAKind(_, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::FullHouse(_, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::ThreeOfAKind(y1, y2, y3) = b {
                    if x1 == y1 && x2 == y2 {
                        return (get_card_value(y3)).cmp(&get_card_value(x3));
                    } else if x1 == y1 {
                        return (get_card_value(y2)).cmp(&get_card_value(x2));
                    }
                    return (get_card_value(y1)).cmp(&get_card_value(x1));
                }
                return Ordering::Less;
            },
            HandType::TwoPair(x1, x2,x3) => {
                if let HandType::FiveOfAKind(_)=b {
                    return Ordering::Greater;
                }
                if let HandType::FourOfAKind(_, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::FullHouse(_, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::ThreeOfAKind(_, _, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::TwoPair(y1, y2, y3) = b {
                    if x1 == y1 {
                        if x2 == y2 {
                            return (get_card_value(y3)).cmp(&get_card_value(x3));
                        }
                        return (get_card_value(y2)).cmp(&get_card_value(x2));
                    }
                    return (get_card_value(y1)).cmp(&get_card_value(x1));
                }
                return Ordering::Less;
            },
            HandType::OnePair(x1, x2, x3, x4) => {
                if let HandType::FiveOfAKind(_)=b {
                    return Ordering::Greater;
                }
                if let HandType::FourOfAKind(_, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::FullHouse(_, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::ThreeOfAKind(_, _, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::TwoPair(_, _, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::OnePair(y1, y2, y3, y4) = b {
                    if x1 == y1 && x2 == y2 && x3 == y3 {
                        return (get_card_value(y4)).cmp(&get_card_value(x4));
                    } else if x1 == y1 && x2 == y2 {
                        return (get_card_value(y3)).cmp(&get_card_value(x3));
                    } else if x1 == y1 {
                        return (get_card_value(y2)).cmp(&get_card_value(x2));
                    }
                    return (get_card_value(y1)).cmp(&get_card_value(x1));
                }
                return Ordering::Less;
            },
            HandType::HighCard(x1, x2, x3, x4, x5) => {
                if let HandType::FiveOfAKind(_)=b {
                    return Ordering::Greater;
                }
                if let HandType::FourOfAKind(_, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::FullHouse(_, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::ThreeOfAKind(_, _, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::TwoPair(_, _, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::OnePair(_, _, _, _)=b {
                    return Ordering::Greater;
                }
                if let HandType::HighCard(y1, y2, y3, y4, y5) = b {
                    if x1==y1 && x2==y2 && x3 == y3 && x4 == y4 {
                        return (get_card_value(y5)).cmp(&get_card_value(x5));
                    } else if x1==y1 && x2==y2 && x3 == y3 {
                        return (get_card_value(y4)).cmp(&get_card_value(x4));
                    } else if x1==y1 && x2==y2 {
                        return (get_card_value(y3)).cmp(&get_card_value(x3));
                    } else if x1==y1 {
                        return (get_card_value(y2)).cmp(&get_card_value(x2));
                    }
                    return (get_card_value(y1)).cmp(&get_card_value(x1));
                }
                return Ordering::Less;
            },
        
    }

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
