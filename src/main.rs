/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

use std::path::PathBuf;

use cells::GameRules;
use clap::Parser;
use color_eyre::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::style::Color;

use crate::app::App;
use crate::event::{Event, EventHandler};
use crate::tui::Tui;
use update::update;

#[derive(Parser, Debug)]
#[command(version, about = "A simple life-like cellular automaton simulator")]
struct Cli {
    /// Simulation rules - a string in Golly/RLE format, defaults to Conway's Game of Life 'B2/S23'
    #[arg(short, long)]
    rules: Option<GameRules>,

    /// Path to the cells' initial state file. If absent a default example preset will be used
    #[arg(short, long)]
    cells_file: Option<PathBuf>,

    /// A color which will be used for drawing alive cells
    #[arg(long, default_value_t=Color::Yellow)]
    cells_color: Color,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    // Create an application.
    let mut app = App::new(cli.rules, cli.cells_file, cli.cells_color)?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(1000);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit() {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                if !app.is_paused() {
                    app.to_next_state()
                };
            }
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
