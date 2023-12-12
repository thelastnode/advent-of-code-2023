use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use console::{style, Term};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CellType {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub struct Grid {
    pub cells: HashMap<Position, CellType>,
    pub size: (usize, usize),
    pub start_position: Position,
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let mut cells = HashMap::new();
        let mut start_position = None;

        for (row, line) in input.lines().enumerate() {
            for (col, cell) in line.char_indices() {
                let position = Position { row, col };
                if cell == 'S' {
                    start_position = Some(position);
                } else {
                    cells.insert(position, CellType::from_char(cell));
                }
            }
        }

        let start_position = start_position.expect("No start position found");
        let grid_size: (usize, usize) =
            (input.lines().count(), input.lines().next().unwrap().len());

        let start_cell_type = Grid::infer_start_cell_type(&cells, &grid_size, &start_position);
        cells.insert(start_position.clone(), start_cell_type);

        Grid {
            cells,
            size: grid_size,
            start_position,
        }
    }

    fn infer_start_cell_type(
        cells: &HashMap<Position, CellType>,
        grid_size: &(usize, usize),
        start_position: &Position,
    ) -> CellType {
        let offsets = Position::offsets()
            .filter_map(|offset| {
                let neighbor = (start_position + offset)?;
                neighbor.is_valid(grid_size).then_some(offset)
            })
            .filter(|valid_offset| {
                let neighbor = (start_position + valid_offset).expect("should be filtered");
                let is_connected = neighbor
                    .connected_pipes(&cells[&neighbor])
                    .any(|x| x == *start_position);

                is_connected
            })
            .sorted()
            .collect_vec();
        match offsets.as_slice() {
            [(-1, 0), (1, 0)] => CellType::Vertical,
            [(0, -1), (0, 1)] => CellType::Horizontal,
            [(-1, 0), (0, 1)] => CellType::BendNE,
            [(-1, 0), (0, -1)] => CellType::BendNW,
            [(0, -1), (1, 0)] => CellType::BendSW,
            [(0, 1), (1, 0)] => CellType::BendSE,
            _ => panic!("Invalid start location"),
        }
    }

    pub fn print(&self, highlight: &HashSet<Position>) -> Result<(), std::io::Error> {
        let term = Term::stdout();
        for row in 0..self.size.0 {
            let mut line = String::new();
            for col in 0..self.size.1 {
                let pos = Position { row, col };
                let c = match self.cells[&pos] {
                    CellType::Vertical => '|',
                    CellType::Horizontal => '-',
                    CellType::BendNE => '⎣',
                    CellType::BendNW => '⎦',
                    CellType::BendSW => '⎤',
                    CellType::BendSE => '⎡',
                    CellType::Ground => '⋅',
                    CellType::Start => unreachable!(),
                };
                if self.start_position == pos {
                    line += &style(c).red().to_string();
                } else if highlight.contains(&pos) {
                    line += &style(c).blue().to_string();
                } else {
                    line += &c.to_string();
                }
            }
            term.write_line(&line)?;
        }
        Ok(())
    }
}

impl Position {
    fn offsets() -> impl Iterator<Item = &'static (isize, isize)> {
        const OFFSETS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
        OFFSETS.iter()
    }

    pub fn neighbors<'a>(
        &'a self,
        grid_size: &'a (usize, usize),
    ) -> impl Iterator<Item = Position> + 'a {
        Self::offsets()
            .map(|(dr, dc)| (self.row as isize + dr, self.col as isize + dc))
            .filter(|&(row, col)| row >= 0 && col >= 0)
            .map(|(row, col)| Position {
                row: row as usize,
                col: col as usize,
            })
            .filter(|position| position.row < grid_size.0 && position.col < grid_size.1)
    }

    pub fn connected_pipes<'a>(
        &'a self,
        cell_type: &CellType,
    ) -> impl Iterator<Item = Position> + 'a {
        cell_type
            .connected_pipe_offsets()
            .into_iter()
            .filter_map(move |offset: (isize, isize)| self + &offset)
    }

    fn is_valid(&self, &(rows, cols): &(usize, usize)) -> bool {
        self.row < rows && self.col < cols
    }
}

impl Add<&(isize, isize)> for &Position {
    type Output = Option<Position>;

    fn add(self, rhs: &(isize, isize)) -> Self::Output {
        Some(Position {
            row: (self.row).checked_add_signed(rhs.0)?,
            col: self.col.checked_add_signed(rhs.1)?,
        })
    }
}

impl CellType {
    fn from_char(c: char) -> Self {
        match c {
            '|' => CellType::Vertical,
            '-' => CellType::Horizontal,
            'L' => CellType::BendNE,
            'J' => CellType::BendNW,
            '7' => CellType::BendSW,
            'F' => CellType::BendSE,
            '.' => CellType::Ground,
            'S' => CellType::Start,
            _ => panic!("Invalid character: {}", c),
        }
    }

    fn connected_pipe_offsets(&self) -> Vec<(isize, isize)> {
        match self {
            CellType::Vertical => vec![(-1, 0), (1, 0)],
            CellType::Horizontal => vec![(0, -1), (0, 1)],
            CellType::BendNE => vec![(-1, 0), (0, 1)],
            CellType::BendNW => vec![(-1, 0), (0, -1)],
            CellType::BendSW => vec![(1, 0), (0, -1)],
            CellType::BendSE => vec![(1, 0), (0, 1)],
            CellType::Ground => vec![],
            CellType::Start => panic!("Start location should be inferred first"),
        }
    }
}
