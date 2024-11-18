use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::Borders;
use ratatui::{
    layout::Rect,
    style::Color,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::shared::{Point, TileType, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::entities::{Map, Player};

#[derive(Debug, Clone)]
pub struct Game {
    map: Map,
    exit: bool,
    player: Player,
    area: Rect,
    cell_char: String,
    player_char: String,
}

// TODO: Separar responsabilidades dessa game em (por exemplo)
// GameProcessor
// GameRender/GameUI/GameView

impl Game {
    pub fn new() -> Game {
        Self {
            map: Map::new(),
            exit: false,
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
            area: Rect::new(0, 0, SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16),
            cell_char: "█".to_string(),
            player_char: "█".to_string(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, self.area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let player = &mut self.player;
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => player.update(-2, 0),
            KeyCode::Right => player.update(2, 0),
            KeyCode::Up => player.update(0, -1),
            KeyCode::Down => player.update(0, 1),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw_cell(&self, line: &mut Line<'_>, point: Point) {
        let floor_span = Span::from(self.cell_char.clone());
        let idx = Map::map_idx(point.x, point.y);
        match self.map.get_tile(idx) {
            TileType::Flor => {
                line.push_span(floor_span);
            }
            TileType::Wall => {
                line.push_span(Span::from("#"));
            }
        }
    }

    fn draw_player(&self, line: &mut Line<'_>) {
        let char = Span::from(self.player_char.clone()).style(Style::default().fg(Color::Black));
        line.push_span(char.clone());
    }
}

impl Widget for &Game {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut screen_content: Text = Text::default();

        for y in 0..(SCREEN_HEIGHT - 1) {
            let mut line: Line = Line::default();
            for x in 0..(SCREEN_WIDTH - 1) {
                let point = Point::new(x, y);
                if Map::equals(point, self.player.position) {
                    self.draw_player(&mut line);
                } else {
                    self.draw_cell(&mut line, point);
                }
            }
            screen_content.push_line(line);
        }

        Paragraph::new(screen_content)
            .style(Style::default().fg(Color::Yellow).bg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Dungeon Rustlers"),
            )
            .render(area, buf);
    }
}
