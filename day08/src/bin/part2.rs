use std::{collections::HashMap, env::args, fs};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
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
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        );
        let (input, (name, (left, right))) =
            separated_pair(alphanumeric1, tag(" = "), left_right_parser)(input)?;

        Ok((input, Node { name, left, right }))
    }

    fn is_start(&self) -> bool {
        self.name.ends_with('A')
    }

    fn is_end(&self) -> bool {
        self.name.ends_with('Z')
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<Direction>, Vec<Node>)> {
    let (input, path) = many1(Direction::parse)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, nodes) = separated_list1(tag("\n"), Node::parse)(input)?;

    assert!(input.is_empty(), "`{input}`");

    Ok((input, (path, nodes)))
}

fn lowest_common_multiple(mut items: Vec<usize>) -> usize {
    let mut divisors: Vec<usize> = Vec::new();

    loop {
        if items.iter().all(|&x| x == 1) {
            break;
        }
        let lowest_divisor = (2..)
            .find(|&divisor| items.iter().any(|x| x % divisor == 0))
            .unwrap();
        divisors.push(lowest_divisor);

        for x in items.iter_mut() {
            if *x % lowest_divisor == 0 {
                *x /= lowest_divisor;
            }
        }
    }
    divisors.into_iter().product()
}

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let (_, (path, nodes)) = parse(&input).unwrap();

    let nodes: HashMap<&str, Node> = nodes.into_iter().map(|n| (n.name, n)).collect();

    let loop_sizes = nodes
        .values()
        .filter(|node| node.is_start())
        .map(|start| {
            let steps_to_end = path
                .iter()
                .cycle()
                .scan(start.name, |current, dir| {
                    if nodes[current].is_end() {
                        return None;
                    }
                    let next = match dir {
                        Direction::Left => nodes[current].left,
                        Direction::Right => nodes[current].right,
                    };
                    *current = next;
                    Some(next)
                })
                .count();
            steps_to_end
        })
        .collect_vec();

    dbg!(&loop_sizes);
    dbg!(lowest_common_multiple(loop_sizes));
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
