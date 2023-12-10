use std::{env::args, fs};

use day09::{parse_sensors, process_sensor};

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let sensors = parse_sensors(&input);

    let result = sensors
        .iter()
        .map(|sensor| process_sensor(sensor, |x| *x.first().unwrap(), |offset, x| x - offset))
        .sum::<i64>();
    println!("{result}");
}
