use std::{collections::HashSet, env::args, fs};

use day03::*;

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let elements = parse(&input);
    let symbol_positions: HashSet<&Pos> = elements
        .iter()
        .filter_map(|e| match e {
            Element::Symbol(Symbol { char: _, position }) => Some(position),
            _ => None,
        })
        .collect();

    let result: u32 = elements
        .iter()
        .filter_map(|e| match e {
            Element::Number(n) => Some(n),
            _ => None,
        })
        .filter(|number| {
            number.positions.iter().any(|pos| {
                pos.neighbors()
                    .iter()
                    .any(|neighbor| symbol_positions.contains(neighbor))
            })
        })
        .map(|number| number.number)
        .sum();
    dbg!(result);
}
