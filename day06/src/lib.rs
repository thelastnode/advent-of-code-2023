use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
pub struct Race {
    pub time: i64,
    pub distance: i64,
}

impl Race {
    pub fn ways_to_win(&self) -> i64 {
        // Find two points where this is true:
        // (race.time - hold_time) * hold_time == race.distance
        //
        // Restructured for quadratic formula:
        // -hold_time^2 + race.time * hold_time - race.distance = 0
        let a: f64 = -1f64;
        let b: f64 = self.time as f64;
        let c: f64 = (-self.distance) as f64;

        let sqrt = (b.powi(2) - 4f64 * a * c).sqrt();
        let (left, right) = ((-b + sqrt) / (2f64 * a), (-b - sqrt) / (2f64 * a));

        let left_exact = (right.floor() - right).abs() < f64::EPSILON;
        let right_exact = (left.ceil() - left).abs() < f64::EPSILON;
        let edge_matches = (left_exact as i64) + (right_exact as i64);

        (right.floor() - left.ceil()) as i64 + 1 - edge_matches
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, digit1)(input)?;
    let (input, _) = tag("\nDistance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, digit1)(input)?;

    let races = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        })
        .collect();

    Ok((input, races))
}
