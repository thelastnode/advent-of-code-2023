use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub cubes: HashMap<String, u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub hands: Vec<Hand>,
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

pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(newline, parse_game)(input)?;
    Ok((input, games))
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
