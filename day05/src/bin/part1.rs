use std::{env::args, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct MapEntry {
    source: i64,
    destination: i64,
    length: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Map<'a> {
    from: &'a str,
    to: &'a str,
    entries: Vec<MapEntry>,
}

impl MapEntry {
    fn parse(input: &str) -> IResult<&str, MapEntry> {
        let (input, destination) = terminated(complete::i64, tag(" "))(input)?;
        let (input, source) = terminated(complete::i64, tag(" "))(input)?;
        let (input, length) = complete::i64(input)?;

        Ok((
            input,
            MapEntry {
                source,
                destination,
                length,
            },
        ))
    }
}

impl<'a> Map<'a> {
    fn parse(input: &'a str) -> IResult<&str, Map<'a>> {
        let (input, from) = alpha1(input)?;
        let (input, _) = tag("-to-")(input)?;
        let (input, to) = alpha1(input)?;
        let (input, _) = tag(" map:\n")(input)?;
        let (input, entries) = separated_list1(tag("\n"), MapEntry::parse)(input)?;
        // let (input, _) = tag("\n")(input)?;

        Ok((input, Map { from, to, entries }))
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(tag(" "), complete::i64)(input)?;
    let (input, _) = tag("\n\n")(input)?;

    Ok((input, seeds))
}

fn parse(input: &str) -> (Vec<i64>, Vec<Map>) {
    dbg!(&input);
    let (input, seeds) = parse_seeds(input).unwrap();
    let (input, maps) = separated_list1(tag("\n\n"), Map::parse)(input).unwrap();

    assert!(input.is_empty());

    (seeds, maps)
}

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    dbg!(parse(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "seed-to-soil map:\n\
            50 98 2\n\
            52 50 48";
        assert_eq!(
            Map::parse(input),
            Ok((
                "",
                Map {
                    from: "seed",
                    to: "soil",
                    entries: vec![
                        MapEntry {
                            destination: 50,
                            source: 98,
                            length: 2,
                        },
                        MapEntry {
                            destination: 52,
                            source: 50,
                            length: 48,
                        },
                    ],
                }
            ))
        );
    }
}
