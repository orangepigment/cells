use color_eyre::eyre::{Result, WrapErr};
use std::{path::Path, str::FromStr};

use cells::{Automaton, Cell, GameRules};

#[derive(Debug)]
pub struct App {
    automaton: Automaton,
    should_quit: bool,
    paused: bool,
}

impl App {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<App> {
        Automaton::from_file(path)
            .wrap_err("Failed to load automaton definition from {path}")
            .map(|automaton| App {
                automaton,
                should_quit: false,
                paused: false,
            })
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn cells(&self) -> &[Cell] {
        self.automaton.cells()
    }

    pub fn height(&self) -> usize {
        self.automaton.height()
    }

    pub fn width(&self) -> usize {
        self.automaton.width()
    }

    pub fn generation(&self) -> u64 {
        self.automaton.generation()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn to_next_state(&mut self) {
        self.automaton.to_next_state();
    }
}

/// For demo usage
impl Default for App {
    fn default() -> Self {
        // TOAD 6*6
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
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 3
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            // Row 4
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            // Row 5
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 6
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ];

        let rules = GameRules::from_str("B3/S23").unwrap();
        let automaton = Automaton::preset(cells, 6, 6, rules).unwrap();

        App {
            automaton,
            should_quit: false,
            paused: false,
        }
    }
}
