use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs,
};

use day03::*;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let elements = parse(&input);
    let possible_gear_positions: HashSet<&Pos> = elements
        .iter()
        .flat_map(|e| match e {
            Element::Symbol(Symbol {
                char: '*',
                position,
            }) => Some(position),
            _ => None,
        })
        .collect();

    let mut gear_candidates: HashMap<Pos, Vec<Number>> = HashMap::new();

    let numbers = elements.iter().filter_map(|e| match e {
        Element::Number(n) => Some(n),
        _ => None,
    });
    numbers.for_each(|number| {
        let neighboring_positions = number.positions.iter().flat_map(|pos| pos.neighbors());
        let adjacent_gears = neighboring_positions
            .filter(|neighbor| possible_gear_positions.contains(&neighbor))
            .unique();
        adjacent_gears.for_each(|gear_position| {
            gear_candidates
                .entry(gear_position)
                .or_default()
                .push(number.clone());
        })
    });

    let result = gear_candidates
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers.iter().map(|number| number.number).product::<u32>())
        .sum::<u32>();
    dbg!(result);
}
