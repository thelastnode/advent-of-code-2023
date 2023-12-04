use std::{env::args, fs};

use day04::Card;

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let cards: Vec<Card> = input.lines().map(Card::parse).collect();

    let mut counts = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        for j in 0..card.match_count() {
            counts[i + j + 1] += counts[i];
        }
    }
    let result: usize = counts.iter().sum();

    dbg!(result);
}
