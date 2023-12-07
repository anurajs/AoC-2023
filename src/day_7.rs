use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub enum Card {
    Joker,
    Number(usize),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum HandStrength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, Ord, Eq)]
pub struct Hand {
    pub bid: usize,
    pub hand: [Card; 5],
    pub strength: HandStrength,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.strength.cmp(&other.strength) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                let mut counter = 0;
                while self.hand[counter] == other.hand[counter] {
                    counter += 1;
                }
                Some(self.hand[counter].cmp(&other.hand[counter]))
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Hand {
    fn make_hand(cards: &str) -> [Card; 5] {
        let mut hand: Vec<Card> = Vec::with_capacity(5);
        for card in cards.chars() {
            match card {
                'A' => hand.push(Card::Ace),
                'K' => hand.push(Card::King),
                'Q' => hand.push(Card::Queen),
                'J' => hand.push(Card::Jack),
                'T' => hand.push(Card::Ten),
                x if x.is_numeric() => hand.push(Card::Number(x.to_digit(10).unwrap() as usize)),
                x => {
                    panic!("Expect a card found {x}");
                }
            }
        }

        hand.try_into().unwrap_or_else(|v: Vec<Card>| {
            panic!("Expected a Vec of length {} but it was {}", 5, v.len())
        })
    }

    fn make_hand_part_two(cards: &str) -> [Card; 5] {
        let mut hand: Vec<Card> = Vec::with_capacity(5);
        for card in cards.chars() {
            match card {
                'A' => hand.push(Card::Ace),
                'K' => hand.push(Card::King),
                'Q' => hand.push(Card::Queen),
                'J' => hand.push(Card::Joker),
                'T' => hand.push(Card::Ten),
                x if x.is_numeric() => hand.push(Card::Number(x.to_digit(10).unwrap() as usize)),
                x => {
                    panic!("Expect a card found {x}");
                }
            }
        }

        hand.try_into().unwrap_or_else(|v: Vec<Card>| {
            panic!("Expected a Vec of length {} but it was {}", 5, v.len())
        })
    }

    fn calculate_hand_strength(hand: [Card; 5]) -> HandStrength {
        let mut card_counts = HashMap::new();
        for card in hand {
            *card_counts.entry(card).or_insert(0) += 1;
        }

        let mut strengths = Vec::new();

        for (_, count) in card_counts {
            match count {
                5 => strengths.push(HandStrength::FiveOfAKind),
                4 => strengths.push(HandStrength::FourOfAKind),
                3 => strengths.push(HandStrength::ThreeOfAKind),
                2 => strengths.push(HandStrength::OnePair),
                1 => strengths.push(HandStrength::HighCard),
                _ => {}
            }
        }
        strengths.sort_by(|a, b| b.cmp(a));
        let mut strength = strengths[0];
        if let HandStrength::ThreeOfAKind = strength {
            if let HandStrength::OnePair = strengths[1] {
                strength = HandStrength::FullHouse;
            }
        } else if let HandStrength::OnePair = strength {
            if let HandStrength::OnePair = strengths[1] {
                strength = HandStrength::TwoPair;
            }
        }
        strength
    }

    fn calculate_hand_strength_part_two(cards: [Card; 5]) -> HandStrength {
        let mut strengths: Vec<HandStrength> = Vec::new();
        let possible = [
            Card::Ace,
            Card::King,
            Card::Queen,
            Card::Jack,
            Card::Ten,
            Card::Number(9),
            Card::Number(8),
            Card::Number(7),
            Card::Number(6),
            Card::Number(5),
            Card::Number(4),
            Card::Number(3),
            Card::Number(2),
        ];
        for card in possible {
            let temp = Self::replace_all(cards, Card::Joker, card);
            strengths.push(Self::calculate_hand_strength(temp));
        }

        return *strengths.iter().max().unwrap();
    }

    fn replace_all(mut cards: [Card; 5], a: Card, b: Card) -> [Card; 5] {
        for i in 0..5 {
            if cards[i] == a {
                cards[i] = b;
            }
        }
        cards
    }

    pub fn new(line: &str, part: usize) -> Self {
        let cards = line.split_whitespace().nth(0).unwrap();
        let hand = if part == 1 {
            Self::make_hand(cards)
        } else {
            Self::make_hand_part_two(cards)
        };
        let hand_strength = if part == 1 {
            Self::calculate_hand_strength(hand)
        } else {
            Self::calculate_hand_strength_part_two(hand)
        };
        let bid = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        Hand {
            bid,
            hand,
            strength: hand_strength,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::download_day;

    use super::Hand;

    static SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_one() {
        let content = download_day(2023, 7);
        let mut hands: Vec<_> = content.lines().map(|line| Hand::new(line, 1)).collect();
        hands.sort();
        let res: usize = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) * hand.bid)
            .sum();
        println!("Part One: {res}");
    }

    #[test]
    fn part_two() {
        let content = download_day(2023, 7);
        let mut hands: Vec<_> = content.lines().map(|line| Hand::new(line, 2)).collect();
        hands.sort();
        let res: usize = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) * hand.bid)
            .sum();
        println!("Part Two: {res}");
    }

    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let mut hands: Vec<_> = content.lines().map(|line| Hand::new(line, 1)).collect();
        hands.sort();
        let res: usize = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) * hand.bid)
            .sum();
        // println!("{hands:?}");
        assert_eq!(res, 6440);
    }

    #[test]
    fn part_two_sample() {
        let content = SAMPLE;
        let mut hands: Vec<_> = content.lines().map(|line| Hand::new(line, 2)).collect();
        hands.sort();
        let res: usize = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) * hand.bid)
            .sum();
        // println!("{hands:?}");
        assert_eq!(res, 5905);
    }
}