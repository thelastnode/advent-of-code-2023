use std::{env::args, fs};

fn parse_digit_positions(line: &str) -> Vec<(usize, u32)> {
    const WRITTEN_DIGITS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let digits = line
        .char_indices()
        .filter(|(_, c)| c.is_numeric())
        .map(|(i, c)| (i, c.to_digit(10).unwrap()));
    let written = WRITTEN_DIGITS.iter().enumerate().flat_map(|(i, dig)| {
        line.match_indices(dig)
            .map(move |(m, _)| (m, u32::try_from(i + 1).unwrap()))
    });

    let mut vec: Vec<_> = written.chain(digits).collect();
    vec.sort_by_key(|(i, _)| *i);
    vec
}

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let sum = input
        .lines()
        .map(|line| {
            let positions = parse_digit_positions(line);
            let (first, last) = (positions.first().unwrap().1, positions.last().unwrap().1);
            first * 10 + last
        })
        .sum::<u32>();
    println!("{}", sum);
}
