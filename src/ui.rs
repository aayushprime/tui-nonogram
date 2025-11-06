use crate::{    game::{Game},    puzzle::{Puzzle, CellState},};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_game_over_screen(frame: &mut Frame) {
    let message = "Game Over!\n\nYou made 3 mistakes.\n\nPress 'space' to return to the menu.";
    let message_widget = Paragraph::new(message)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Game Over"));
    frame.render_widget(message_widget, frame.size());
}

pub fn render_win_screen(frame: &mut Frame) {
    let message = "You win!\n\nPress 'space' to return to the menu.";
    let message_widget = Paragraph::new(message)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Congratulations!"));
    frame.render_widget(message_widget, frame.size());
}

pub fn render_menu(frame: &mut Frame, game: &Game) {
    let difficulties = ["Easy", "Medium", "Hard"];
    let items = [
        "Start Game".to_string(),
        format!("Grid Size: <{}>", game.grid_size),
        format!("Difficulty: <{}>", difficulties[game.difficulty as usize]),
    ];

    let mut lines = vec![];
    for (i, item) in items.iter().enumerate() {
        let style = if i == game.selected_item {
            Style::default().fg(Color::Black).bg(Color::White)
        } else {
            Style::default()
        };
        lines.push(Line::from(Span::styled(item.clone(), style)));
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Menu"));
    frame.render_widget(paragraph, frame.size());
}

pub fn render_puzzle(frame: &mut Frame, puzzle: &Puzzle, is_error: bool, mistakes: u32) {
    let terminal_area = frame.size();

    let max_row_hint_len = puzzle
        .row_hints
        .iter()
        .map(|hints| hints.len())
        .max()
        .unwrap_or(0);
    let max_col_hint_len = puzzle
        .col_hints
        .iter()
        .map(|hints| hints.len())
        .max()
        .unwrap_or(0);

    let puzzle_width = puzzle.width * 2 + 2;
    let puzzle_height = puzzle.height + 2;

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(terminal_area);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(max_row_hint_len as u16 * 2),
            Constraint::Length(puzzle_width as u16),
        ])
        .split(main_layout[0]);

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(max_col_hint_len as u16),
            Constraint::Length(puzzle_height as u16),
        ])
        .split(layout[1]);

    let col_hints_area = vertical_layout[0];
    let puzzle_area = vertical_layout[1];
    let mut row_hints_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(max_col_hint_len as u16),
            Constraint::Length(puzzle_height as u16),
        ])
        .split(layout[0])[1];
    row_hints_area.y += 1;
    row_hints_area.height = row_hints_area.height.saturating_sub(1);


    // Column hints
    let mut col_hint_lines = vec![];
    for i in 0..max_col_hint_len {
        let mut spans = vec![];
        for hints in &puzzle.col_hints {
            if i < hints.len() {
                spans.push(Span::from(format!("{:2}", hints[i])));
            } else {
                spans.push(Span::from("  "));
            }
        }
        col_hint_lines.push(Line::from(spans));
    }
    let col_hints_widget = Paragraph::new(Text::from(col_hint_lines)).alignment(Alignment::Center);
    frame.render_widget(col_hints_widget, col_hints_area);

    // Row hints
    let mut row_hint_lines = vec![];
    for hints in &puzzle.row_hints {
        let mut spans = vec![];
        let padding = max_row_hint_len - hints.len();
        for _ in 0..padding {
            spans.push(Span::from("  "));
        }
        for hint in hints {
            spans.push(Span::from(format!("{:2}", hint)));
        }
        row_hint_lines.push(Line::from(spans));
    }
    let row_hints_widget = Paragraph::new(Text::from(row_hint_lines));
    frame.render_widget(row_hints_widget, row_hints_area);

    // Puzzle
    let mut lines = vec![];
    for (y, row) in puzzle.state.iter().enumerate() {
        let mut spans = vec![];
        for (x, cell) in row.iter().enumerate() {
            let symbol = match cell {
                CellState::Unknown => ". ",
                CellState::X => "X ",
                CellState::O => "o ",
            };
            let style = if (x as u32, y as u32) == puzzle.cursor {
                Style::default().bg(Color::Cyan)
            } else {
                Style::default()
            };
            spans.push(Span::styled(symbol, style));
        }
        lines.push(Line::from(spans));
    }
    let puzzle_text = Text::from(lines);

    let block_style = if is_error {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    };
    let puzzle_widget =
        Paragraph::new(puzzle_text).block(Block::default().borders(Borders::ALL).title("Nonogram").style(block_style));
    frame.render_widget(puzzle_widget, puzzle_area);

    // Mistakes
    let mistakes_widget = Paragraph::new(format!("Mistakes: {}/3", mistakes));
    frame.render_widget(mistakes_widget, main_layout[1]);
}
