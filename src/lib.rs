mod errors;
mod rules;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use crate::errors::{CellsError, Result};

pub use rules::{GameRules, PresetName, RulePresets};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

#[derive(Debug, PartialEq)]
pub struct Automaton {
    cells: Vec<Cell>,
    height: usize,
    width: usize,
    rules: GameRules,
    generation: u64,
}

impl Automaton {
    // TODO: add a setter for cell via col+row/idx?
    pub fn empty(height: usize, width: usize, rules: Option<GameRules>) -> Result<Self> {
        if height < 1 {
            return Err(CellsError::AutomatonInit(
                errors::AutomatonInitError::Height { height },
            ));
        }
        if width < 1 {
            return Err(CellsError::AutomatonInit(
                errors::AutomatonInitError::Width { width },
            ));
        }

        Ok(Automaton {
            cells: vec![Cell::Dead; height * width],
            height,
            width,
            rules: rules.unwrap_or(RulePresets::new().game_of_life()),
            generation: 1,
        })
    }

    pub fn new(
        cells: Vec<Cell>,
        height: usize,
        width: usize,
        rules: Option<GameRules>,
    ) -> Result<Self> {
        if height < 1 {
            return Err(CellsError::AutomatonInit(
                errors::AutomatonInitError::Height { height },
            ));
        }
        if width < 1 {
            return Err(CellsError::AutomatonInit(
                errors::AutomatonInitError::Width { width },
            ));
        }

        let cells_num = cells.len();
        if height * width != cells_num {
            return Err(CellsError::AutomatonInit(
                errors::AutomatonInitError::GridSize {
                    cells_num,
                    height,
                    width,
                },
            ));
        }

        let rules = rules.unwrap_or(RulePresets::new().game_of_life());

        Ok(Automaton {
            cells,
            height,
            width,
            rules,
            generation: 1,
        })
    }

    pub fn from_file<P: AsRef<Path>>(path: P, rules: Option<GameRules>) -> Result<Automaton> {
        if !path
            .as_ref()
            .try_exists()
            .map_err(CellsError::automaton_init_io)?
        {
            return Err(CellsError::automaton_init_io(io::Error::from(
                io::ErrorKind::NotFound,
            )));
        };

        if !path.as_ref().is_file() {
            return Err(CellsError::automaton_init_io(io::Error::other(
                "{path} is not a regular file",
            )));
        }

        let source = File::open(path).map_err(CellsError::automaton_init_io)?;
        let buf_source = BufReader::new(source);

        let mut lines = buf_source.lines();

        // TODO: parse header - will it contain only rules but also expected width and height?
        // TODO: Make header optional? Good idea.
        let rules = rules.unwrap_or(RulePresets::new().game_of_life());

        // Manual handling of the first cells' data line
        if let Some(first_line) = lines
            .next()
            .transpose()
            .map_err(CellsError::automaton_init_io)?
        {
            let mut cells: Vec<Cell> = Vec::new();
            let mut width: usize = 0;

            for ch in first_line.chars() {
                Self::process_char(&mut cells, ch)?;
                width += 1;
            }

            let mut height: usize = 1;
            for (i, l) in lines.enumerate() {
                let mut line_length: usize = 0;
                for ch in l.map_err(CellsError::automaton_init_io)?.chars() {
                    Self::process_char(&mut cells, ch)?;
                    line_length += 1;
                }
                if line_length != width {
                    return Err(CellsError::automaton_init_io(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Line {i}: Inconsistent data line length encountered"),
                    )));
                }

                height += 1;
            }

            Ok(Automaton {
                cells,
                height,
                width,
                rules,
                generation: 1,
            })
        } else {
            Err(CellsError::automaton_init_io(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Expected cells data, but got end of file",
            )))
        }
    }

    // TODO: provide current parser position for debug?
    fn process_char(cells: &mut Vec<Cell>, ch: char) -> Result<()> {
        match ch {
            '0' => {
                cells.push(Cell::Dead);
            }
            '1' => {
                cells.push(Cell::Alive);
            }
            invalid => {
                return Err(CellsError::automaton_init_io(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Only 0 and 1 characters are supported in data lines, but got {invalid}"
                    ),
                )));
            }
        }
        Ok(())
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn to_next_state(&mut self) {
        let mut new_cells: Vec<Cell> = Vec::new();

        for (i, cell) in self.cells.iter().enumerate() {
            let neighbors = self.get_neighbors(i);

            let alive_neighbors: u8 = neighbors
                .iter()
                .filter(|j| match self.cells[**j] {
                    Cell::Dead => false,
                    Cell::Alive => true,
                })
                .count()
                .try_into()
                .expect("Number of neighbors cannot be greater than 8!");

            match cell {
                Cell::Dead => {
                    let new_state = if self.rules.is_born(alive_neighbors) {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    };
                    new_cells.push(new_state);
                }
                Cell::Alive => {
                    let new_state = if self.rules.has_survived(alive_neighbors) {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    };
                    new_cells.push(new_state)
                }
            }
        }

        self.cells = new_cells;
        self.generation += 1;
    }

    /// Returns indices of the cell's neighbors
    fn get_neighbors(&self, cell_idx: usize) -> Vec<usize> {
        let row = cell_idx / self.width;
        let col = cell_idx % self.width;

        let mut neighbors_indices: Vec<usize> = Vec::new();

        // Upper
        if row != 0 {
            if col != 0 {
                neighbors_indices.push(cell_idx - self.width - 1);
            }
            neighbors_indices.push(cell_idx - self.width);
            if col != self.width - 1 {
                neighbors_indices.push(cell_idx - self.width + 1);
            }
        }
        // Left
        if col != 0 {
            neighbors_indices.push(cell_idx - 1);
        }
        // Right
        if col != self.width - 1 {
            neighbors_indices.push(cell_idx + 1);
        }
        // Bottom
        if row != self.height - 1 {
            if col != 0 {
                neighbors_indices.push(cell_idx + self.width - 1);
            }
            neighbors_indices.push(cell_idx + self.width);
            if col != self.width - 1 {
                neighbors_indices.push(cell_idx + self.width + 1);
            }
        }

        neighbors_indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_fs::fixture::FileWriteStr;
    use std::result::Result as StdResult;

    // Automaton constructors' tests
    // TODO:  Add error test cases for all constructors. Or at least for from_file
    #[test]
    fn create_empty_automaton() {
        let mut presets = RulePresets::default();
        let automaton =
            Automaton::empty(4, 3, Some(presets.game_of_life())).expect("Failed to init Automaton");

        let expected = Automaton {
            cells: vec![Cell::Dead; 12],
            height: 4,
            width: 3,
            rules: presets.game_of_life(),
            generation: 1,
        };

        assert_eq!(automaton, expected);
    }

    #[test]
    fn create_new_automaton() {
        let mut presets = RulePresets::default();

        // [Blinker](https://conwaylife.com/wiki/Blinker) pattern
        let cells = vec![
            // Row 1
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 2
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 3
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 4
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 5
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ];

        let automaton =
            Automaton::new(cells.clone(), 5, 6, None).expect("Failed to init Automaton");

        let expected = Automaton {
            cells,
            height: 5,
            width: 6,
            rules: presets.game_of_life(),
            generation: 1,
        };

        assert_eq!(automaton, expected);
    }

    #[test]
    fn from_file_automaton() -> StdResult<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("example.cells")?;
        // [Blinker](https://conwaylife.com/wiki/Blinker) pattern
        file.write_str(
            "00000
00100
00100
00100
00000",
        )?;

        let mut presets = RulePresets::default();

        let cells = vec![
            // Row 1
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 2
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            // Row 3
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            // Row 4
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            // Row 5
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ];

        let automaton =
            Automaton::from_file(file.path(), None).expect("Failed to load Automaton from file");

        let expected = Automaton {
            cells,
            height: 5,
            width: 5,
            rules: presets.game_of_life(),
            generation: 1,
        };

        assert_eq!(automaton, expected);

        Ok(())
    }

    // TODO: tests for to_next_state function
    #[test]
    fn test_next_state() {
        let mut presets = RulePresets::default();

        // [Blinker](https://conwaylife.com/wiki/Blinker) pattern
        let initial_cells = vec![
            // Row 1
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 2
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 3
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 4
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 5
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ];

        let next_state_cells = vec![
            // Row 1
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 2
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 3
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            // Row 4
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 5
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ];

        let mut automaton =
            Automaton::new(initial_cells, 5, 6, None).expect("Failed to init Automaton");
        automaton.to_next_state();

        let expected = Automaton {
            cells: next_state_cells,
            height: 5,
            width: 6,
            rules: presets.game_of_life(),
            generation: 2,
        };

        assert_eq!(automaton, expected);
    }

    #[test]
    fn get_neighbors() {
        let automaton = Automaton::empty(3, 3, None).expect("Failed to init automaton");

        assert_eq!(automaton.get_neighbors(0), vec![1, 3, 4]);
        assert_eq!(automaton.get_neighbors(1), vec![0, 2, 3, 4, 5]);
        assert_eq!(automaton.get_neighbors(2), vec![1, 4, 5]);
        assert_eq!(automaton.get_neighbors(3), vec![0, 1, 4, 6, 7]);
        assert_eq!(automaton.get_neighbors(4), vec![0, 1, 2, 3, 5, 6, 7, 8]);
        assert_eq!(automaton.get_neighbors(5), vec![1, 2, 4, 7, 8]);
        assert_eq!(automaton.get_neighbors(6), vec![3, 4, 7]);
        assert_eq!(automaton.get_neighbors(7), vec![3, 4, 5, 6, 8]);
        assert_eq!(automaton.get_neighbors(8), vec![4, 5, 7]);
    }
}
