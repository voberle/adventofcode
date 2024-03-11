use std::io::{self, Read};

use itertools::Itertools;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::{Alignment, Buffer, Color, Frame, Line, Rect, Span, Style, Stylize, Widget},
    widgets::block::{Position, Title},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use fxhash::FxHashMap;
use intcode::IntcodeComputer;

mod terminal;

#[derive(Debug, Clone, Copy)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TileType {
    fn new(v: i64) -> Self {
        match v {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::Paddle,
            4 => TileType::Ball,
            _ => panic!("Invalid tile type {}", v),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn from(x: i64, y: i64) -> Self {
        Self {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }
}

enum Output {
    Tile(Pos, TileType),
    Score(usize),
    End,
}

impl Output {
    fn tile(x: i64, y: i64, t: i64) -> Self {
        Output::Tile(Pos::from(x, y), TileType::new(t))
    }

    fn score(s: i64) -> Self {
        Output::Score(s.try_into().unwrap())
    }

    fn end() -> Self {
        Output::End
    }
}

fn last_output(computer: &mut IntcodeComputer) -> Output {
    if let Some(third_val) = computer.io.get_output() {
        let y = computer.io.get_output().unwrap();
        let x = computer.io.get_output().unwrap();
        if x == -1 && y == 0 {
            Output::score(third_val)
        } else {
            Output::tile(x, y, third_val)
        }
    } else {
        Output::end()
    }
}

// Stores the tiles and the score.
struct Display {
    tiles: FxHashMap<Pos, TileType>,
    score: usize,
    // The max x and y, which we can save once we have a first set of tiles.
    max_dims: Option<Pos>,
}

impl Display {
    fn empty() -> Self {
        Self {
            tiles: FxHashMap::default(),
            score: 0,
            max_dims: None,
        }
    }

    fn update_borders(&mut self) {
        if self.max_dims.is_none() {
            let mut min_pos = Pos::new(usize::MAX, usize::MAX);
            let mut max_pos = Pos::new(usize::MIN, usize::MIN);
            for pos in self.tiles.keys() {
                min_pos.x = min_pos.x.min(pos.x);
                max_pos.x = max_pos.x.max(pos.x);
                min_pos.y = min_pos.y.min(pos.y);
                max_pos.y = max_pos.y.max(pos.y);
            }
            assert_eq!(min_pos.x, 0);
            assert_eq!(min_pos.y, 0);
            self.max_dims = Some(max_pos);
        }
    }

    fn update(&mut self, computer: &mut IntcodeComputer) {
        computer.exec();

        loop {
            let out = last_output(computer);
            match out {
                Output::Tile(pos, tile) => {
                    self.tiles.insert(pos, tile);
                }
                Output::Score(s) => {
                    self.score = s;
                }
                Output::End => {
                    break;
                }
            }
        }
        self.update_borders();
    }
}

fn block_tiles_count(computer: &IntcodeComputer) -> usize {
    let mut computer = computer.clone();
    let mut display = Display::empty();
    display.update(&mut computer);
    display
        .tiles
        .values()
        .filter(|t| matches!(t, TileType::Block))
        .count()
}

fn parse_computer_input(input: &str) -> Vec<i64> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn high_score(computer: &IntcodeComputer) -> usize {
    const WINNING_INPUT: &str = "resources/winning_computer_input";

    let mut computer = computer.clone();
    // Enable game mode
    computer.write_mem(0, 2);

    let mut display = Display::empty();

    let input = std::fs::read_to_string(WINNING_INPUT).expect("Missing saved file");
    let saved_inputs: Vec<i64> = parse_computer_input(&input);

    display.update(&mut computer);
    for i in saved_inputs {
        computer.io.add_input(i);
        display.update(&mut computer);
    }
    assert!(computer.is_halted());

    display.score
}

/// The UI for the game.
struct App {
    computer: IntcodeComputer,
    display: Display,
    exit: bool,
    saved_inputs: Vec<i64>,
}

impl App {
    const SAVED_FILE: &'static str = "resources/saved_keypresses";

    fn new(computer: &IntcodeComputer, use_saved: bool) -> Self {
        let mut app = Self {
            computer: computer.clone(),
            display: Display::empty(),
            exit: false,
            saved_inputs: Vec::new(),
        };

        // Enable game mode
        app.computer.write_mem(0, 2);

        if use_saved {
            app.load_input_and_replay();
        }
        app
    }

    fn load_input_and_replay(&mut self) {
        if let Ok(input) = std::fs::read_to_string(Self::SAVED_FILE) {
            self.saved_inputs = parse_computer_input(&input);

            self.display.update(&mut self.computer);
            for i in &self.saved_inputs {
                self.computer.io.add_input(*i);
                self.display.update(&mut self.computer);
            }
        }
    }

    fn save_input(&self) {
        let s = self.saved_inputs.iter().join(",");
        std::fs::write(Self::SAVED_FILE, s).unwrap();
    }

    /// runs the application's main loop until the user quits
    fn run(&mut self, terminal: &mut terminal::Tui) -> io::Result<()> {
        while !self.exit {
            self.display.update(&mut self.computer);

            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.left(),
            KeyCode::Right => self.right(),
            KeyCode::Char(' ') => self.neutral(),
            KeyCode::Char('s') => self.save_input(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn left(&mut self) {
        self.saved_inputs.push(-1);
        self.computer.io.add_input(-1);
    }

    fn right(&mut self) {
        self.saved_inputs.push(1);
        self.computer.io.add_input(1);
    }

    fn neutral(&mut self) {
        self.saved_inputs.push(0);
        self.computer.io.add_input(0);
    }
}

impl Widget for &App {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Intcode Game. ".bold());

        let mut text = Vec::new();

        for y in 0..=self.display.max_dims.unwrap().y {
            let mut spans_vec = vec![];
            for x in 0..=self.display.max_dims.unwrap().x {
                const EMPTY: &str = "\u{2B1C}";
                let c = if let Some(tile) = self.display.tiles.get(&Pos::new(x, y)) {
                    match tile {
                        TileType::Empty => EMPTY,
                        TileType::Wall => "\u{1F7EB}",  // brown square
                        TileType::Block => "\u{1F7E6}", // blue square
                        TileType::Paddle => "\u{1F7E8}", // yellow square
                        TileType::Ball => "\u{1F534}",  // red circle
                    }
                } else {
                    EMPTY
                };
                let span = Span::styled(c.to_string(), Style::default().fg(Color::Black));
                spans_vec.push(span);
            }
            text.push(Line::from(spans_vec));
        }

        let score_str = if self.computer.is_halted() {
            format!(" Game Over! Score: {} ", self.display.score)
        } else {
            format!(" Score: {} ", self.display.score)
        };
        let score = Title::from(Line::from(score_str.red().bold()));

        let instructions = Title::from(Line::from(vec![
            "<Left>".blue().bold(),
            " ".into(),
            "<Right>".blue().bold(),
            " ".into(),
            "<Space>".blue().bold(),
            " ".into(),
            "<Q> ".blue().bold(),
        ]));

        let p = Paragraph::new(text)
            .block(
                Block::default()
                    .title(title.alignment(Alignment::Center))
                    .title(
                        score
                            .alignment(Alignment::Center)
                            .position(Position::Bottom),
                    )
                    .title(
                        instructions
                            .alignment(Alignment::Right)
                            .position(Position::Bottom),
                    )
                    .borders(Borders::ALL)
                    .border_style(Style::default().black()),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });

        // Hard-coded size to make it pretty
        let size = Rect::new(0, 0, 83, 24);
        p.render(size, buf);
        // Alternatively you can render it in the whole terminal with:
        // p.render(_area, buf);
    }
}

fn main() -> io::Result<()> {
    let param = std::env::args().nth(1).unwrap_or_default();
    if !param.is_empty() {
        // Not reading from stdin in this case, as it messes up with crossterm.
        let input = std::fs::read_to_string("resources/input").expect("Unable to read input file");
        let computer = IntcodeComputer::build(&input);

        let mut terminal = terminal::init()?;
        let mut app = App::new(&computer, param == "saved");

        let app_result = app.run(&mut terminal);
        terminal::restore()?;
        return app_result;
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", block_tiles_count(&computer));
    println!("Part 2: {}", high_score(&computer));

    Ok(())
}
