mod game;
mod puzzle;
mod tui;
mod ui;

use color_eyre::Result;
use game::run;
use tui::{init_terminal, restore_terminal};

fn main() -> Result<()> {
    let terminal = init_terminal()?;
    run(terminal)?;
    restore_terminal()
}