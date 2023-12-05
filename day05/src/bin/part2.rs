use std::{collections::HashMap, env::args, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Span {
    start: i64,
    length: i64,
}

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

    fn apply(&self, span: &Span) -> (Vec<Span>, Vec<Span>) {
        let span_range = span.start..(span.start + span.length);
        let entry_range = self.source..(self.source + self.length);
        let entry_offset = self.destination - self.source;

        // [ ] < >
        // < > [ ]
        let is_entirely_before = span_range.end <= entry_range.start;
        let is_entirely_after = span_range.start >= entry_range.end;

        if is_entirely_before || is_entirely_after {
            return (vec![span.clone()], vec![]);
        }

        // < [ ] >
        let is_entirely_contained =
            span_range.start >= entry_range.start && span_range.end <= entry_range.end;
        if is_entirely_contained {
            return (
                vec![],
                vec![Span {
                    start: span_range.start + entry_offset,
                    length: span.length,
                }],
            );
        }

        // [ < > ]
        let is_super_span_of_entry =
            span_range.start < entry_range.start && span_range.end > entry_range.end;
        if is_super_span_of_entry {
            return (
                vec![
                    Span {
                        start: span_range.start,
                        length: entry_range.start - span_range.start,
                    },
                    Span {
                        start: entry_range.end,
                        length: span_range.end - entry_range.end,
                    },
                ],
                vec![Span {
                    start: entry_range.start + entry_offset,
                    length: entry_range.end - entry_range.start,
                }],
            );
        }
        // [ < ] >
        let is_left_overlap = span_range.start < entry_range.start;
        if is_left_overlap {
            return (
                vec![Span {
                    start: span_range.start,
                    length: entry_range.start - span_range.start,
                }],
                vec![Span {
                    start: entry_range.start + entry_offset,
                    length: span_range.end - entry_range.start,
                }],
            );
        }
        // < [ > ]
        let is_right_overlap = span_range.start >= entry_range.start;
        if is_right_overlap {
            return (
                vec![Span {
                    start: span_range.start + entry_offset,
                    length: entry_range.end - span_range.start,
                }],
                vec![Span {
                    start: entry_range.end,
                    length: span_range.end - entry_range.end,
                }],
            );
        }

        unreachable!();
    }
}

impl<'a> Map<'a> {
    fn parse(input: &'a str) -> IResult<&str, Map<'a>> {
        let (input, from) = alpha1(input)?;
        let (input, _) = tag("-to-")(input)?;
        let (input, to) = alpha1(input)?;
        let (input, _) = tag(" map:\n")(input)?;
        let (input, entries) = separated_list1(tag("\n"), MapEntry::parse)(input)?;

        Ok((input, Map { from, to, entries }))
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Span>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(
        tag(" "),
        separated_pair(complete::i64, tag(" "), complete::i64),
    )(input)?;
    let (input, _) = tag("\n\n")(input)?;

    Ok((
        input,
        seeds
            .into_iter()
            .map(|(start, length)| Span { start, length })
            .collect(),
    ))
}

fn parse(input: &str) -> (Vec<Span>, Vec<Map>) {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (input, maps) = separated_list1(tag("\n\n"), Map::parse)(input).unwrap();

    assert!(input.is_empty());

    (seeds, maps)
}

fn follow_maps(map_by_type: &HashMap<&str, Map>, seed: &Span, destination_type: &str) -> i64 {
    let mut spans = vec![seed.clone()];
    let mut current_type = "seed";

    while current_type != destination_type {
        let map = map_by_type.get(current_type).unwrap();
        let mut old_spans = spans.clone();
        let mut new_spans = Vec::new();

        for entry in map.entries.iter() {
            let mut spans_to_retry = Vec::new();

            while let Some(span) = old_spans.pop() {
                let (unaffected, affected) = entry.apply(&span);
                new_spans.extend(affected);
                spans_to_retry.extend(unaffected);
            }

            old_spans = spans_to_retry;
        }
        spans = old_spans.clone();
        spans.extend(new_spans);
        current_type = map.to;
    }

    spans.iter().map(|span| span.start).min().unwrap()
}

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let (seeds, maps) = parse(&input);
    let map_by_type: HashMap<&str, Map> =
        HashMap::from_iter(maps.into_iter().map(|map| (map.from, map)));

    const DEST: &str = "location";
    let result = seeds
        .iter()
        .map(|seed| follow_maps(&map_by_type, seed, DEST))
        .min()
        .unwrap();
    dbg!(result);
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
