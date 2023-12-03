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
        .filter_map(|e| match e {
            Element::Symbol(Symbol { char, position }) => {
                if *char == '*' {
                    Some(position)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    let mut gears: HashMap<Pos, Vec<Number>> = HashMap::new();

    elements
        .iter()
        .filter_map(|e| match e {
            Element::Number(n) => Some(n),
            _ => None,
        })
        .for_each(|number| {
            let neighbors = number.positions.iter().flat_map(|pos| pos.neighbors());
            neighbors
                .filter(|neighbor| possible_gear_positions.contains(&neighbor))
                .unique()
                .for_each(|gear_position| {
                    gears.entry(gear_position).or_default().push(number.clone());
                })
        });

    let result = gears
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers.iter().map(|number| number.number).product::<u32>())
        .sum::<u32>();
    dbg!(result);
}
