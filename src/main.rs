use color_eyre::Result;
use ratatui::DefaultTerminal;

mod pong;
mod app;
mod input;

use crate::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal: DefaultTerminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
