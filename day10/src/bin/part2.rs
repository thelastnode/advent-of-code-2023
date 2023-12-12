use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::args,
    error::Error,
    fs,
};

use day10::{CellType, Grid, Position};
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
        new_cells.insert(neighbor, CellType::Horizontal);
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

    let mut queue = VecDeque::from_iter([grid.start_position.clone()]);
    let mut visited = HashSet::new();

    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }

        let connected_pipes = pos.connected_pipes(&grid.cells[&pos]).collect_vec();
        visited.insert(pos);

        for connected_pipe in connected_pipes {
            if !visited.contains(&connected_pipe) {
                queue.push_back(connected_pipe);
            }
        }
    }

    let grid = double_grid(&grid, &visited);

    let grounds_along_edge = grid
        .cells
        .iter()
        .filter(|(&Position { row, col }, _)| {
            row == 0 || col == 0 || (row == grid.size.0 - 1) || (col == grid.size.1 - 1)
        })
        .filter_map(|(pos, cell_type)| (*cell_type == CellType::Ground).then_some(pos.clone()));

    let mut queue = VecDeque::from_iter(grounds_along_edge);
    let mut visited = HashSet::new();

    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }

        let neighbors = pos
            .neighbors(&grid.size)
            .filter(|neighbor| grid.cells[neighbor] == CellType::Ground)
            .collect_vec();
        visited.insert(pos);

        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    let mut ground_count = 0;
    let mut inside_count = 0;

    for (row, col) in (0..grid.size.0)
        .step_by(2)
        .cartesian_product((0..grid.size.1).step_by(2))
    {
        let pos = Position { row, col };

        if grid.cells[&pos] == CellType::Ground {
            ground_count += 1;

            if !visited.contains(&pos) {
                inside_count += 1;
            }
        }
    }

    println!("{inside_count} / {ground_count}");

    Ok(())
}
