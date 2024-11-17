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

use crate::{build_units_list, do_action, grid::Grid, is_full_unit_dead};

type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

pub fn fancy(map: &Grid) -> Result<()> {
    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    // We don't need raw mode, as we don't capture input. Not having raw mode allows ctrl-c to work for example.
    // enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut map = map.clone();
    let mut units = build_units_list(&map);

    let mut combat_rounds = 0;
    'outer: loop {
        units.sort_by_key(|u| u.position);
        for i in 0..units.len() {
            if is_full_unit_dead(&units) {
                break 'outer;
            }
            do_action(&mut map, &mut units, i);
        }
        combat_rounds += 1;
        units.retain(|u| !u.is_dead());

        let mut text = Vec::new();
        for row in 0..map.rows {
            let mut spans_vec = vec![];
            for p in row * map.cols..(row + 1) * map.cols {
                let c = map.values[p];
                let mut span = Span::styled(format!("{c}"), Style::default().fg(Color::Black));
                if c == '.' {
                    span.style = Style::default().fg(Color::Gray);
                }
                if c == 'G' {
                    span.style = Style::default().fg(Color::Red);
                }
                if c == 'E' {
                    span.style = Style::default().fg(Color::Green);
                }
                spans_vec.push(span);
            }
            text.push(Line::from(spans_vec));
        }
        let p = Paragraph::new(text)
            .block(
                Block::default()
                    .title(format!("Round {combat_rounds}"))
                    .borders(Borders::ALL)
                    .border_style(Style::default().black()),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });

        terminal.draw(|f| {
            let size = Rect::new(
                0,
                0,
                u16::try_from(map.cols).unwrap() + 2,
                u16::try_from(map.rows).unwrap() + 2,
            );
            f.render_widget(p, size);
        })?;

        thread::sleep(Duration::from_millis(100));
    }

    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen)?;
    // disable_raw_mode()?;
    Ok(())
}
