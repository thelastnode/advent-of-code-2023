use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub fn neighbors(&self) -> Vec<Pos> {
        const OFFSETS: &[isize] = &[-1isize, 0, 1];
        OFFSETS
            .iter()
            .cartesian_product(OFFSETS)
            .map(|(dr, dc)| Pos {
                row: (self.row as isize + dr) as usize,
                col: (self.col as isize + dc) as usize,
            })
            .filter(|pos: &Pos| pos != self)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Number {
    pub number: u32,
    pub positions: Vec<Pos>,
}

impl Number {
    pub fn is_adjacent_to(&self, locations: &HashSet<&Pos>) -> bool {
        self.positions
            .iter()
            .flat_map(|pos| pos.neighbors())
            .any(|neighbor| locations.contains(&neighbor))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub char: char,
    pub position: Pos,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Element {
    Number(Number),
    Symbol(Symbol),
}

fn parse_leading_number(line: &str, row: usize, col: usize) -> Number {
    let digits: Vec<_> = line
        .char_indices()
        .skip(col)
        .take_while(|(_, char)| char.is_ascii_digit())
        .collect();
    let number = digits
        .iter()
        .map(|(_, digit)| digit.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, digit)| digit * (10u32).pow((digits.len() - i - 1) as u32))
        .sum();
    return Number {
        number,
        positions: digits.iter().map(|(i, _)| Pos { row, col: *i }).collect(),
    };
}

fn parse_line(line: &str, row: usize) -> Vec<Element> {
    let mut visited = HashSet::new();
    line.char_indices()
        .flat_map(|(col, char)| {
            if visited.contains(&col) {
                return None;
            }
            match char {
                '0'..='9' => {
                    let number = parse_leading_number(line, row, col);
                    for pos in &number.positions {
                        visited.insert(pos.col);
                    }

                    Some(Element::Number(number))
                }
                '.' => None,
                _ => Some(Element::Symbol(Symbol {
                    char,
                    position: Pos { row, col },
                })),
            }
        })
        .collect()
}

pub fn parse(input: &str) -> Vec<Element> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| parse_line(line, row))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("123", 3),
            vec![Element::Number(Number {
                number: 123,
                positions: vec![
                    Pos { row: 3, col: 0 },
                    Pos { row: 3, col: 1 },
                    Pos { row: 3, col: 2 },
                ]
            }),]
        );
        assert_eq!(
            parse_line(".123...456", 3),
            vec![
                Element::Number(Number {
                    number: 123,
                    positions: vec![
                        Pos { row: 3, col: 1 },
                        Pos { row: 3, col: 2 },
                        Pos { row: 3, col: 3 },
                    ],
                }),
                Element::Number(Number {
                    number: 456,
                    positions: vec![
                        Pos { row: 3, col: 7 },
                        Pos { row: 3, col: 8 },
                        Pos { row: 3, col: 9 },
                    ],
                }),
            ]
        );
    }
}
