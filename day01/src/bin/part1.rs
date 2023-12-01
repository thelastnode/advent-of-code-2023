use std::{env::args, fs};

use day01::*;

fn digits(line: &str) -> Vec<u32> {
    line.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let lines = parse(&input);
    let sum = lines
        .iter()
        .map(|line| {
            let digs = digits(line);
            let [first, last] = [digs.first().unwrap(), digs.last().unwrap()];
            first * 10 + last
        })
        .sum::<u32>();
    println!("{}", sum);
}
