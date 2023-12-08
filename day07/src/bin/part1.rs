use std::{cell::OnceCell, collections::HashMap, env::args, fs};

use itertools::Itertools;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Card(char);

impl Card {
    fn new(c: char) -> Card {
        Card(c)
    }

    fn value(&self) -> usize {
        match CARD_ORDERING.get(&self.0) {
            Some(&i) => i,
            None => self
                .0
                .to_digit(10)
                .expect("should be digit")
                .try_into()
                .expect("should convert into usize"),
        }
    }
}

static CARD_ORDERING: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    ['A', 'K', 'Q', 'J', 'T']
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (x, i + 10))
        .collect()
});

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

static HAND_TYPE_ORDERING: Lazy<HashMap<HandType, usize>> = Lazy::new(|| {
    [
        HandType::FiveOfAKind,
        HandType::FourOfAKind,
        HandType::FullHouse,
        HandType::ThreeOfAKind,
        HandType::TwoPair,
        HandType::OnePair,
        HandType::HighCard,
    ]
    .into_iter()
    .rev()
    .enumerate()
    .map(|(i, x)| (x, i))
    .collect()
});

impl HandType {
    fn value(&self) -> usize {
        *HAND_TYPE_ORDERING.get(self).unwrap()
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: i32,
    lazy_hand_type: OnceCell<HandType>,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: i32) -> Self {
        Self {
            cards,
            bid,
            lazy_hand_type: OnceCell::new(),
        }
    }

    fn hand_type(&self) -> &HandType {
        self.lazy_hand_type
            .get_or_init(|| self.identify_hand_type())
    }

    fn identify_hand_type(&self) -> HandType {
        let counts = self
            .cards
            .iter()
            .counts()
            .into_values()
            .sorted()
            .rev()
            .collect_vec();

        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            [1, ..] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid && self.hand_type() == other.hand_type()
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(other.hand_type())
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| line.split_at(line.find(' ').unwrap()))
        .map(|(cards, bid)| {
            Hand::new(
                cards.chars().map(Card::new).collect(),
                bid.trim().parse().unwrap(),
            )
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let hands = parse(&input);
    let result: usize = hands
        .into_iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum();
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_cmp() {
        assert!(Card::new('A') > Card::new('7'));
    }

    #[test]
    fn test_hand_type() {
        assert!(Hand::new(parse_cards("KKKKK"), 0).hand_type() == &HandType::FiveOfAKind);
        assert!(Hand::new(parse_cards("KTJJT"), 0).hand_type() == &HandType::TwoPair);
        assert!(Hand::new(parse_cards("T55J5"), 0).hand_type() == &HandType::ThreeOfAKind);
    }

    #[test]
    fn test_hand_cmp() {
        assert!(Hand::new(parse_cards("KTJJT"), 0) > Hand::new(parse_cards("32T3K"), 0));
        assert!(Hand::new(parse_cards("KK677"), 0) > Hand::new(parse_cards("KTJJT"), 0));
        assert!(Hand::new(parse_cards("T55J5"), 0) > Hand::new(parse_cards("KTJJT"), 0));
    }

    fn parse_cards(s: &str) -> Vec<Card> {
        s.chars().map(Card::new).collect()
    }
}
