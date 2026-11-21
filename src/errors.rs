use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, CellsError>;

#[derive(Debug)]
pub enum CellsError {
    GameRulesParsing(String),
    // TODO: remove nested error and use plain enum variants?
    AutomatonInit(AutomatonInitError),
}

impl CellsError {
    pub fn automaton_init_io(error: io::Error) -> CellsError {
        CellsError::AutomatonInit(AutomatonInitError::IO(error))
    }
}

impl fmt::Display for CellsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellsError::GameRulesParsing(raw_rules) => {
                write!(
                    f,
                    "Invalid rules string '{raw_rules}'. A string in Golly/RLE format is expected"
                )
            }
            CellsError::AutomatonInit(automaton_init_error) => automaton_init_error.fmt(f),
        }
    }
}

impl error::Error for CellsError {}

#[derive(Debug)]
pub enum AutomatonInitError {
    GridSize {
        cells_num: usize,
        height: usize,
        width: usize,
    },
    Height {
        height: usize,
    },
    Width {
        width: usize,
    },
    IO(io::Error),
}

impl fmt::Display for AutomatonInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutomatonInitError::GridSize {
                cells_num,
                height,
                width,
            } => write!(
                f,
                "A grid with height={height} and width={width} must have {} cells, but got {cells_num}",
                height * width
            ),
            AutomatonInitError::Height { height } => {
                write!(f, "height must be positive, but got {height}")
            }
            AutomatonInitError::Width { width } => {
                write!(f, "height must be positive, but got {width}")
            }
            AutomatonInitError::IO(error) => error.fmt(f),
        }
    }
}

impl error::Error for AutomatonInitError {}
