use std::{env::args, fs};

fn digits(line: &str) -> Vec<u32> {
    line.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let sum = input
        .lines()
        .map(|line| {
            let digs = digits(line);
            let [first, last] = [digs.first().unwrap(), digs.last().unwrap()];
            first * 10 + last
        })
        .sum::<u32>();
    println!("{}", sum);
}
