use std::io;

use crossterm::{
    event::Event::Key,
    event::KeyCode::Esc,
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen}
};
use crossterm::terminal::LeaveAlternateScreen;

use tui::{
    backend::CrosstermBackend,
    backend::Backend,
    Frame,
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
    style::Style,
    style::Color::*,
    Terminal
};

use tui::text::Span;
use tui::widgets::Clear;

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

fn clear<B: Backend>(f: &mut Frame<B>) {
    f.render_widget(Clear, f.size());
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| { clear(f) })?;

    loop {
        terminal.draw(|f| { ui(f) })?;

        if let Key(e) = crossterm::event::read()? {
            if e.code == Esc {
                break;
            }
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;

    disable_raw_mode()?;

    Ok(())
}
