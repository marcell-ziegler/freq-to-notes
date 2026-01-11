use crate::app::{App, run_app};

mod app;
mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let result = run_app(&mut terminal, &mut app);
    ratatui::restore();
    result
}
