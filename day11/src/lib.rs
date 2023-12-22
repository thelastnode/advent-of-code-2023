use itertools::Itertools;

fn find_positions(grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &c)| c)
                .map(move |(col, _)| (row, col))
        })
        .collect_vec()
}

pub fn get_distance_sum(input: &str, scaling_factor: i64) -> i64 {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let rows_to_double = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| !row.iter().any(|&c| c))
        .map(|(i, _)| i)
        .rev() // reverse to avoid shifting indices
        .collect_vec();

    let cols_to_double = (0..grid[0].len())
        .filter(|&col| !grid.iter().any(|row| row[col]))
        .rev()
        .collect_vec();

    let positions = find_positions(&grid);

    let result = (0..(positions.len() - 1))
        .flat_map(|i| ((i + 1)..positions.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (r1, c1) = positions[i];
            let (r2, c2) = positions[j];

            let dist = (r1 as i64 - r2 as i64).abs() + (c1 as i64 - c2 as i64).abs();
            let row_crossings = rows_to_double
                .iter()
                .filter(|&&row| row > r1.min(r2) && row < r1.max(r2))
                .count() as i64;
            let col_crossings = cols_to_double
                .iter()
                .filter(|&&col| col > c1.min(c2) && col < c1.max(c2))
                .count() as i64;
            dist + (row_crossings + col_crossings) * (scaling_factor - 1)
        })
        .sum::<i64>();
    result
}
