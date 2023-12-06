use std::{env::args, fs};

use day06::{parse, Race};

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let (_, races) = parse(&input).unwrap();

    let result: i64 = races.iter().map(Race::ways_to_win).product();

    dbg!(result);
}
