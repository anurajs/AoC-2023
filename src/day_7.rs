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

pub trait ParseHand {
    fn make_hand(&self, cards: &str) -> [Card; 5] {
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
    fn calculate_hand_strength(&self, hand: [Card; 5]) -> HandStrength {
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
#[derive(Clone, Copy)]
struct DefaultHandParser {}
impl ParseHand for DefaultHandParser {}
#[derive(Clone, Copy)]
struct JokerHandParser {}
impl JokerHandParser {
    fn replace_all(mut cards: [Card; 5], a: Card, b: Card) -> [Card; 5] {
        for i in 0..5 {
            if cards[i] == a {
                cards[i] = b;
            }
        }
        cards
    }
}
impl ParseHand for JokerHandParser {
    fn make_hand(&self, cards: &str) -> [Card; 5] {
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
    fn calculate_hand_strength(&self, hand: [Card; 5]) -> HandStrength {
        let default_parser = DefaultHandParser {};
        let mut strengths: Vec<HandStrength> = Vec::new();
        let mut possible: Vec<_> =
            [Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Ten].to_vec();
        possible.extend((2..10).map(Card::Number));
        for card in possible {
            let temp = Self::replace_all(hand, Card::Joker, card);
            strengths.push(default_parser.calculate_hand_strength(temp));
        }

        return *strengths.iter().max().unwrap();
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Hand {
    pub fn new(line: &str, parser: impl ParseHand) -> Self {
        let cards = line.split_whitespace().nth(0).unwrap();
        let hand = parser.make_hand(cards);
        let hand_strength = parser.calculate_hand_strength(hand);
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

    use crate::{
        day_7::{DefaultHandParser, Hand, JokerHandParser},
        download_day,
    };

    const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_one() {
        let parser = DefaultHandParser {};
        let content = download_day(2023, 7);
        let mut hands: Vec<_> = content
            .lines()
            .map(|line| Hand::new(line, parser))
            .collect();
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
        let parser = JokerHandParser {};
        let content = download_day(2023, 7);
        let mut hands: Vec<_> = content
            .lines()
            .map(|line| Hand::new(line, parser))
            .collect();
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
        let parser = DefaultHandParser {};
        let content = SAMPLE;
        let mut hands: Vec<_> = content
            .lines()
            .map(|line| Hand::new(line, parser))
            .collect();
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
        let parser = JokerHandParser {};
        let content = SAMPLE;
        let mut hands: Vec<_> = content
            .lines()
            .map(|line| Hand::new(line, parser))
            .collect();
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
