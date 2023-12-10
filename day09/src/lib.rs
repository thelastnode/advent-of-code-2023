use itertools::Itertools;

pub fn parse_sensors(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

pub fn process_sensor(
    sensor: &Vec<i64>,
    get_element: fn(sensor: &Vec<i64>) -> i64,
    fold: fn(offset: i64, x: &i64) -> i64,
) -> i64 {
    let mut firsts = vec![get_element(sensor)];
    let mut current = sensor.clone();

    loop {
        let diffs = current
            .iter()
            .zip(current.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect_vec();

        if diffs.iter().all(|&x| x == 0) {
            break;
        }
        firsts.push(get_element(&diffs));
        current = diffs;
    }

    firsts.iter().rev().fold(0, fold)
}
