use std::{collections::HashMap, env::args, fs};

use day02::{parse, Game};

fn is_possible(max_cubes: &HashMap<&str, u32>, game: &Game) -> bool {
    game.hands.iter().all(|hand| {
        hand.cubes
            .iter()
            .all(|(color, count)| count <= max_cubes.get(color.as_str()).unwrap())
    })
}

fn main() {
    let max_cubes: HashMap<&str, u32> = vec![("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect();

    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let (input, games) = parse(&input).unwrap();
    assert_eq!(input.len(), 0);
    let result: u32 = games
        .into_iter()
        .filter(|g| is_possible(&max_cubes, g))
        .map(|g| g.id)
        .sum();

    println!("{}", result);
}
