use std::{env::args, fs};

use day02::parse;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let (input, games) = parse(&input).unwrap();
    assert_eq!(input.len(), 0);

    let result: u32 = games
        .into_iter()
        .map(|game| {
            let cube_counts = game
                .hands
                .into_iter()
                .flat_map(|hand| hand.cubes)
                .into_group_map()
                .into_values()
                .map(|counts| counts.into_iter().max().unwrap())
                .collect_vec();
            match cube_counts.len() {
                3 => cube_counts.iter().product(),
                _ => 0,
            }
        })
        .sum();

    println!("{}", result);
}
