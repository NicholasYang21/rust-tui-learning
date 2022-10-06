use std::{io};
use tui::{
    backend::CrosstermBackend,
    backend::Backend,
    Frame,
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{event::read, event::{EnableMouseCapture}, execute, terminal::{enable_raw_mode, EnterAlternateScreen}, event};
use crossterm::event::KeyCode::Esc;

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref()
        )
        .split(f.size());

    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let block = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            ui(f)
        })?;

        if let event::Event::Key(event) = read()? {
            if event.code == Esc {
                break;
            }
        }
    }

    Ok(())
}