use std::{env::args, fs};

use day11::get_distance_sum;

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let result = get_distance_sum(&input, 1_000_000);
    println!("{result}");
}
