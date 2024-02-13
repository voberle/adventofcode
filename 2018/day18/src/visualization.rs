use std::{io::stdout, thread, time::Duration};

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

use crate::{advance_one_minute, Area, Grid};

type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

pub fn fancy(lumber_collection: &Grid) -> Result<()> {
    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    // We don't need raw mode, as we don't capture input. Not having raw mode allows ctrl-c to work for example.
    // enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut grid = lumber_collection.clone();

    let mut minute = 0;
    loop {
        advance_one_minute(&mut grid);
        minute += 1;
        if minute == 1000 {
            break;
        }

        let mut text = Vec::new();
        for row in 0..grid.rows {
            let mut spans_vec = vec![];
            for p in row * grid.cols..(row + 1) * grid.cols {
                let c = grid.values[p];
                let mut span = Span::styled(format!("{}", c), Style::default().fg(Color::Black));
                if c == Area::OpenGround {
                    span.style = Style::default().fg(Color::Gray);
                }
                if c == Area::Tree {
                    span.style = Style::default().fg(Color::LightGreen);
                }
                if c == Area::Lumberyard {
                    span.style = Style::default().fg(Color::Blue);
                }
                spans_vec.push(span);
            }
            text.push(Line::from(spans_vec));
        }
        let p = Paragraph::new(text)
            .block(
                Block::default()
                    .title(format!("Minute {}", minute))
                    .borders(Borders::ALL)
                    .border_style(Style::default().black()),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });

        terminal.draw(|f| {
            let size = Rect::new(0, 0, grid.cols as u16 + 2, grid.rows as u16 + 2);
            f.render_widget(p, size);
        })?;

        thread::sleep(Duration::from_millis(100));
    }

    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen)?;
    // disable_raw_mode()?;
    Ok(())
}
