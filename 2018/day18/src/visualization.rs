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
    // How fast it should go (decrease for faster)
    const SLEEP_TIME: Duration = Duration::from_millis(25);

    crossterm::execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut grid = lumber_collection.clone();

    let mut minute = 0;
    loop {
        advance_one_minute(&mut grid);
        minute += 1;
        // Don't let it run forever (ctrl-c works, as raw mode isn't enabled)
        if minute == 10000 {
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
            let size = Rect::new(
                0,
                0,
                u16::try_from(grid.cols).unwrap() + 2,
                u16::try_from(grid.rows).unwrap() + 2,
            );
            f.render_widget(p, size);
        })?;

        thread::sleep(SLEEP_TIME);
    }

    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
