use std::{env::args, fs};

use day06::{parse, Race};

fn concat_digits(digits: impl Iterator<Item = i64>) -> i64 {
    digits
        .map(|digit| digit.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let (_, races) = parse(&input).unwrap();

    let race = Race {
        time: concat_digits(races.iter().map(|race| race.time)),
        distance: concat_digits(races.iter().map(|race| race.distance)),
    };

    dbg!(race.ways_to_win());
}
