use std::{
    collections::{HashMap, HashSet},
    env::args,
    error::Error,
    fs,
};

use day10::{bfs, CellType, Grid, Position};
use itertools::Itertools;

fn double_grid(grid: &Grid, pipes: &HashSet<Position>) -> Grid {
    let mut new_cells = grid
        .cells
        .iter()
        .filter(|(pos, _)| pipes.contains(pos))
        .map(|(Position { row, col }, cell_type)| {
            (
                Position {
                    row: row * 2,
                    col: col * 2,
                },
                cell_type.clone(),
            )
        })
        .collect::<HashMap<Position, CellType>>();

    // add neighboring pipes
    let neighboring_pipes = new_cells
        .iter()
        .flat_map(|(pos, cell_type)| pos.connected_pipes(cell_type))
        .collect_vec();
    for neighbor in neighboring_pipes {
        new_cells.insert(neighbor, CellType::Horizontal); // any cell type is fine
    }

    let grounds_to_fill = (0..(grid.size.0 * 2))
        .cartesian_product(0..(grid.size.1 * 2))
        .map(|(row, col)| Position { row, col })
        .filter(|pos| !new_cells.contains_key(pos))
        .collect_vec();
    for pos in grounds_to_fill {
        new_cells.insert(pos, CellType::Ground);
    }

    Grid {
        cells: new_cells,
        size: (grid.size.0 * 2, grid.size.1 * 2),
        start_position: Position {
            row: grid.start_position.row * 2,
            col: grid.start_position.col * 2,
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(args().nth(1).unwrap())?;
    let grid = Grid::parse(&input);

    let grid = {
        let visited = bfs(
            [grid.start_position.clone()].into_iter(),
            |pos| pos.connected_pipes(&grid.cells[pos]).collect_vec(),
            |pos| pos.clone(),
        );

        double_grid(&grid, &visited.into_values().collect())
    };

    let grounds_along_edge = grid
        .cells
        .iter()
        .filter(|(&Position { row, col }, _)| {
            row == 0 || col == 0 || (row == grid.size.0 - 1) || (col == grid.size.1 - 1)
        })
        .filter_map(|(pos, cell_type)| (*cell_type == CellType::Ground).then_some(pos.clone()));

    let get_neighboring_ground_cells = |pos: &Position| {
        pos.neighbors(&grid.size)
            .filter(|neighbor| grid.cells[neighbor] == CellType::Ground)
            .collect_vec()
    };

    let outside_ground_cells: HashSet<Position> = {
        let get_visit_key = |pos: &Position| pos.clone();
        let visited = bfs(
            grounds_along_edge,
            get_neighboring_ground_cells,
            get_visit_key,
        );
        visited.into_values().collect()
    };

    let grid_coords = (0..grid.size.0)
        .step_by(2)
        .cartesian_product((0..grid.size.1).step_by(2));
    let (ground_count, inside_count) = grid_coords
        .map(|(row, col)| Position { row, col })
        .map(|pos| {
            let is_ground = grid.cells[&pos] == CellType::Ground;
            let is_inside = !outside_ground_cells.contains(&pos);
            (is_ground, is_inside)
        })
        .fold(
            (0, 0),
            |(ground_count, inside_count), (is_ground, is_inside)| {
                (
                    ground_count + if is_ground { 1 } else { 0 },
                    inside_count + if is_ground && is_inside { 1 } else { 0 },
                )
            },
        );

    println!("{inside_count} / {ground_count}");

    Ok(())
}
