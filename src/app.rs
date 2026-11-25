use color_eyre::eyre::{Result, WrapErr};
use ratatui::style::Color;
use std::path::PathBuf;

use cells::{Automaton, Cell, GameRules};

#[derive(Debug)]
pub struct App {
    automaton: Automaton,
    cells_color: Color,
    should_quit: bool,
    paused: bool,
}

impl App {
    pub fn new(
        rules: Option<GameRules>,
        cells_file: Option<PathBuf>,
        cells_color: Color,
    ) -> Result<Self> {
        let automaton = match cells_file {
            Some(cf) => Automaton::from_file(cf, rules)
                .wrap_err("Failed to load automaton definition from {path}"),
            None => Automaton::new(App::figure_eight(), 10, 10, rules)
                .wrap_err("Failed to load automaton preset"),
        }?;

        Ok(App {
            automaton,
            cells_color,
            should_quit: false,
            paused: false,
        })
    }

    /// Default configuration [Figure Eight](https://conwaylife.com/wiki/Figure_eight)
    fn figure_eight() -> Vec<Cell> {
        vec![
            // Row 1
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
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
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 3
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
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
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            // Row 6
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 7
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 8
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 9
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            // Row 10
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ]
    }

    pub fn cells_color(&self) -> Color {
        self.cells_color
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

/// A default app settings for demo usage
impl Default for App {
    fn default() -> Self {
        App::new(None, None, Color::Yellow)
            .expect("Default app should be initialized without errors")
    }
}
