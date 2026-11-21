use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    // TODO: add mouse click handling
    // TODO: add Ctrl-z support
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('p') | KeyCode::Char('P') if !app.is_paused() => app.pause(),
        KeyCode::Char('r') | KeyCode::Char('R') if app.is_paused() => app.resume(),
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        _ => {}
    };
}
