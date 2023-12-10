use std::{collections::HashMap, env::args, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(input: &str) -> IResult<&str, Direction> {
        let (input, direction) = alt((tag("L"), tag("R")))
            .map(|c| match c {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            })
            .parse(input)?;
        Ok((input, direction))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl Node<'_> {
    fn parse(input: &str) -> IResult<&str, Node> {
        let left_right_parser = delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        );
        let (input, (name, (left, right))) =
            separated_pair(alpha1, tag(" = "), left_right_parser)(input)?;

        Ok((input, Node { name, left, right }))
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<Direction>, Vec<Node>)> {
    let (input, path) = many1(Direction::parse)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, nodes) = separated_list1(tag("\n"), Node::parse)(input)?;

    assert!(input.is_empty(), "`{input}`");

    Ok((input, (path, nodes)))
}

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let (_, (path, nodes)) = parse(&input).unwrap();

    let nodes: HashMap<&str, Node> = nodes.into_iter().map(|n| (n.name, n)).collect();
    const START: &str = "AAA";
    const END: &str = "ZZZ";

    let mut current = START;
    let mut count = 0;

    for dir in path.iter().cycle() {
        let next = match dir {
            Direction::Left => nodes[current].left,
            Direction::Right => nodes[current].right,
        };
        current = next;
        count += 1;

        if current == END {
            break;
        }
    }

    println!("{count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Node::parse("AAA = (BBB, CCC)"),
            Ok((
                "",
                Node {
                    name: "AAA",
                    left: "BBB",
                    right: "CCC",
                }
            ))
        );
    }
}
