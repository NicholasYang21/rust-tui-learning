use std::{io};
use tui::{
    backend::CrosstermBackend,
    backend::Backend,
    Frame,
    widgets::{Block, Borders, Clear},
    layout::{Layout, Constraint, Direction},
    style::Style,
    style::Color::*,
    Terminal
};
use crossterm::{event::read, event::{EnableMouseCapture}, execute, terminal::{enable_raw_mode, EnterAlternateScreen}, event};
use crossterm::event::KeyCode::Esc;
use tui::text::Span;

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
        .title(Span::styled("Styled Title",
                            Style::default().fg(Rgb(0xff, 0xc6, 0x6d))))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Red));

    f.render_widget(block, chunks[0]);

    let block = Block::default()
        .title(Span::styled("Styled Title 2",
                            Style::default().fg(Green)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(White).bg(Red));

    f.render_widget(block, chunks[1]);
}

fn clear_screen<B: Backend>(f: &mut Frame<B>) {
    f.render_widget(Clear, f.size());
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| { clear_screen(f); })?;

    loop {
        terminal.draw(|f| { ui(f) })?;

        if let event::Event::Key(event) = read()? {
            if event.code == Esc {
                break;
            }
        }
    }

    Ok(())
}