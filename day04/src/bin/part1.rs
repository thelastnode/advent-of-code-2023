use std::{env::args, fs};

use day04::Card;

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let cards: Vec<Card> = input.lines().map(Card::parse).collect();

    let result = cards
        .iter()
        .map(Card::match_count)
        .map(|count| match count {
            0 => 0,
            _ => 2_i32.pow(count as u32 - 1),
        })
        .sum::<i32>();
    dbg!(result);
}
