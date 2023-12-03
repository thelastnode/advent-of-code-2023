use std::{collections::HashSet, env::args, fs};

use day03::*;

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let elements = parse(&input);
    let symbol_positions: HashSet<&Pos> = elements
        .iter()
        .filter_map(|e| match e {
            Element::Symbol(Symbol { position, .. }) => Some(position),
            _ => None,
        })
        .collect();

    let numbers = elements.iter().filter_map(|e| match e {
        Element::Number(n) => Some(n),
        _ => None,
    });
    let result: u32 = numbers
        .filter(|number| number.is_adjacent_to(&symbol_positions))
        .map(|number| number.number)
        .sum();
    dbg!(result);
}
