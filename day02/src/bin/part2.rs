use std::{collections::HashMap, env::args, fs};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cubes: HashMap<String, u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

fn parse_cube(input: &str) -> IResult<&str, (String, u32)> {
    let (input, count) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    Ok((input, (color.to_string(), count.parse().unwrap())))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cubes) = separated_list1(tag(", "), parse_cube)(input)?;
    Ok((
        input,
        Hand {
            cubes: cubes.into_iter().collect(),
        },
    ))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, hands) = separated_list1(tag("; "), parse_hand)(input)?;
    Ok((
        input,
        Game {
            id: id.parse().unwrap(),
            hands,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(newline, parse_game)(input)?;
    Ok((input, games))
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand() {
        assert_eq!(parse_cube("3 blue"), Ok(("", ("blue".to_string(), 3))));
        assert_eq!(
            parse_hand("3 blue, 2 red, 1 green"),
            Ok((
                "",
                Hand {
                    cubes: vec![
                        ("blue".to_string(), 3),
                        ("red".to_string(), 2),
                        ("green".to_string(), 1),
                    ]
                    .into_iter()
                    .collect(),
                }
            ))
        );
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            parse_game("Game 1: 3 blue, 2 red, 1 green; 2 red, 1 green, 3 blue"),
            Ok((
                "",
                Game {
                    id: 1,
                    hands: vec![
                        Hand {
                            cubes: vec![
                                ("blue".to_string(), 3),
                                ("red".to_string(), 2),
                                ("green".to_string(), 1),
                            ]
                            .into_iter()
                            .collect(),
                        },
                        Hand {
                            cubes: vec![
                                ("red".to_string(), 2),
                                ("green".to_string(), 1),
                                ("blue".to_string(), 3),
                            ]
                            .into_iter()
                            .collect(),
                        },
                    ],
                }
            ))
        );
    }
}
