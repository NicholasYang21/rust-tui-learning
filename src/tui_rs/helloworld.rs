use std::io;

use crossterm::{
    event::Event::Key,
    event::KeyCode::Esc,
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};

use tui::{
    backend::{ CrosstermBackend, Backend },
    style::{ Style, Color::* },
    widgets::{ Block, Borders, Clear },
    layout::{ Layout, Constraint, Direction },
    text::{ Span },

    Frame,
    Terminal,
};

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
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

pub fn start() -> Result<(), io::Error> {
    enable_raw_mode()?;

    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut stdout = io::stdout();
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