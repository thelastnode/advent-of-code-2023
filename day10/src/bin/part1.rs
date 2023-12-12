use std::{
    collections::{HashMap, VecDeque},
    env::args,
    fs,
};

use day10::Grid;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let grid = Grid::parse(&input);

    let mut queue = VecDeque::from([(grid.start_position, 0)]);
    let mut visited = HashMap::new();

    while let Some((pos, steps)) = queue.pop_front() {
        if visited.contains_key(&pos) {
            continue;
        }

        let neighbors = pos.connected_pipes(&grid.cells[&pos]).collect_vec();
        visited.insert(pos, steps);

        for neighbor in neighbors {
            if !visited.contains_key(&neighbor) {
                queue.push_back((neighbor, steps + 1));
            }
        }
    }

    let (farthest_pos, steps) = visited.iter().max_by_key(|(_, &steps)| steps).unwrap();
    println!(
        "{} to get to ({}, {})",
        steps, farthest_pos.row, farthest_pos.col
    );
}
