use cells::Cell;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout},
    style::Stylize,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let ui_block = Block::default()
        .title("Cellular Automaton")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .yellow();

    let control_tooltip = if app.is_paused() {
        "`r` to resume"
    } else {
        "`p` for pause"
    };
    let tooltip = Paragraph::new(format!(
        "
        Press {control_tooltip}, `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Generation: {}",
        app.generation()
    ))
    .yellow()
    .centered();

    let inner_area = ui_block.inner(frame.area());

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .horizontal_margin(1);
    let [_, grid_area, tooltip_area] = layout.areas(inner_area);

    // Grid layout with square content (at best effort - depends on actual terminal dimensions)
    let square_height =
        (grid_area.height / app.height() as u16).min(grid_area.width / app.width() as u16 / 2);
    let square_width = square_height * 2;

    let col_constraints = (0..app.width()).map(|_| Constraint::Length(square_width));
    let row_constraints = (0..app.height()).map(|_| Constraint::Length(square_height));

    let horizontal = Layout::horizontal(col_constraints)
        .spacing(1)
        .flex(Flex::Center);
    let vertical = Layout::vertical(row_constraints)
        .spacing(1)
        .flex(Flex::Center);

    let grid_cells = grid_area
        .layout_vec(&vertical)
        .into_iter()
        .flat_map(|row| row.layout_vec(&horizontal));

    frame.render_widget(ui_block, frame.area());

    let automaton_cells = app.cells();
    for (i, grid_cell) in grid_cells.enumerate() {
        if let Cell::Alive = automaton_cells[i] {
            frame.render_widget(Block::new().on_yellow(), grid_cell);
        }
    }

    frame.render_widget(tooltip, tooltip_area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    #[test]
    fn ui_render() {
        let mut app = App::default();

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal.draw(|frame| render(&mut app, frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }
}
