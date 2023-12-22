use std::{env::args, fs};

use day10::{bfs, Grid, Position};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let grid = Grid::parse(&input);

    let get_neighbors = |(pos, step): &(Position, i32)| {
        pos.connected_pipes(&grid.cells[pos])
            .map(|pos| (pos, step + 1))
            .collect_vec()
    };
    let get_visit_key = |(pos, _): &(Position, i32)| pos.clone();

    let visited = bfs(
        [(grid.start_position.clone(), 0i32)].into_iter(),
        get_neighbors,
        get_visit_key,
    );

    let (farthest_pos, steps) = visited.values().max_by_key(|(_, steps)| steps).unwrap();
    println!(
        "{} to get to ({}, {})",
        steps, farthest_pos.row, farthest_pos.col
    );
}
