use crate::{
    puzzle::{create_puzzle, Puzzle, CellState, calculate_hint_for_line},
    ui::{render_menu, render_puzzle, render_win_screen, render_game_over_screen},
};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;

pub enum Screen {
    Menu,
    Puzzle,
    Win,
    GameOver,
}

pub struct Game {
    pub screen: Screen,
    pub puzzle: Puzzle,
    pub selected_item: usize,
    pub grid_size: u32,
    pub difficulty: u32,
    pub error_flash: bool,
    pub mistakes: u32,
}

pub fn run(mut terminal: Terminal<impl Backend>) -> Result<()> {
    let mut game = Game {
        screen: Screen::Menu,
        puzzle: create_puzzle(5, 5),
        selected_item: 0,
        grid_size: 5,
        difficulty: 0,
        error_flash: false,
        mistakes: 0,
    };

    loop {
        match game.screen {
            Screen::Menu => {
                terminal.draw(|frame| render_menu(frame, &game))?;
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => {
                            if game.selected_item == 0 {
                                game.puzzle = create_puzzle(game.grid_size, game.grid_size);
                                game.screen = Screen::Puzzle;
                                game.mistakes = 0;
                            }
                        }
                        KeyCode::Char('j') => {
                            game.selected_item = (game.selected_item + 1) % 3;
                        }
                        KeyCode::Char('k') => {
                            game.selected_item = if game.selected_item == 0 {
                                2
                            } else {
                                game.selected_item - 1
                            };
                        }
                        KeyCode::Char('h') => match game.selected_item {
                            1 => {
                                if game.grid_size > 5 {
                                    game.grid_size -= 1;
                                }
                            }
                            2 => {
                                if game.difficulty > 0 {
                                    game.difficulty -= 1;
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Char('l') => match game.selected_item {
                            1 => {
                                if game.grid_size < 15 {
                                    game.grid_size += 1;
                                }
                            }
                            2 => {
                                if game.difficulty < 2 {
                                    game.difficulty += 1;
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            Screen::Puzzle => {
                terminal.draw(|frame| render_puzzle(frame, &game.puzzle, game.error_flash, game.mistakes))?;
                game.error_flash = false;
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => game.screen = Screen::Menu,
                        KeyCode::Char('h') => {
                            if game.puzzle.cursor.0 > 0 {
                                game.puzzle.cursor.0 -= 1;
                            }
                        }
                        KeyCode::Char('j') => {
                            if game.puzzle.cursor.1 < game.puzzle.height - 1 {
                                game.puzzle.cursor.1 += 1;
                            }
                        }
                        KeyCode::Char('k') => {
                            if game.puzzle.cursor.1 > 0 {
                                game.puzzle.cursor.1 -= 1;
                            }
                        }
                        KeyCode::Char('l') => {
                            if game.puzzle.cursor.0 < game.puzzle.width - 1 {
                                game.puzzle.cursor.0 += 1;
                            }
                        }
                        KeyCode::Char('x') => {
                            let (x, y) = game.puzzle.cursor;
                            if game.puzzle.solution[y as usize][x as usize] {
                                game.puzzle.state[y as usize][x as usize] = CellState::X;
                                check_and_fill_solved(&mut game.puzzle);
                                if is_solved(&game.puzzle) {
                                    game.screen = Screen::Win;
                                }
                            } else {
                                game.error_flash = true;
                                game.mistakes += 1;
                                if game.mistakes >= 3 {
                                    game.screen = Screen::GameOver;
                                }
                            }
                        }
                        KeyCode::Char('o') => {
                            let (x, y) = game.puzzle.cursor;
                            if !game.puzzle.solution[y as usize][x as usize] {
                                game.puzzle.state[y as usize][x as usize] = CellState::O;
                            } else {
                                game.error_flash = true;
                                game.mistakes += 1;
                                if game.mistakes >= 3 {
                                    game.screen = Screen::GameOver;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Screen::Win => {
                terminal.draw(|frame| render_win_screen(frame))?;
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => game.screen = Screen::Menu,
                        _ => {}
                    }
                }
            }
            Screen::GameOver => {
                terminal.draw(|frame| render_game_over_screen(frame))?;
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(' ') => game.screen = Screen::Menu,
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn is_solved(puzzle: &Puzzle) -> bool {
    for y in 0..puzzle.height as usize {
        for x in 0..puzzle.width as usize {
            if (puzzle.state[y][x] == CellState::X) != puzzle.solution[y][x] {
                return false;
            }
        }
    }
    true
}

fn check_and_fill_solved(puzzle: &mut Puzzle) {
    // Check rows
    for y in 0..puzzle.height as usize {
        let row_state: Vec<bool> = (0..puzzle.width as usize)
            .map(|x| puzzle.state[y][x] == CellState::X)
            .collect();
        let hints = calculate_hint_for_line(&row_state);
        if hints == puzzle.row_hints[y] {
            for x in 0..puzzle.width as usize {
                if puzzle.state[y][x] == CellState::Unknown {
                    puzzle.state[y][x] = CellState::O;
                }
            }
        }
    }

    // Check columns
    for x in 0..puzzle.width as usize {
        let col_state: Vec<bool> = (0..puzzle.height as usize)
            .map(|y| puzzle.state[y][x] == CellState::X)
            .collect();
        let hints = calculate_hint_for_line(&col_state);
        if hints == puzzle.col_hints[x] {
            for y in 0..puzzle.height as usize {
                if puzzle.state[y][x] == CellState::Unknown {
                    puzzle.state[y][x] = CellState::O;
                }
            }
        }
    }
}
