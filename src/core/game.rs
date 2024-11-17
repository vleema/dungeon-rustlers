use std::vec;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

use crate::shared::{Point, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::entities::{Map, Player};

#[derive(Debug, Clone)]
pub struct Game {
    map: Map,
    exit: bool,
    player: Player,
    area: Rect,
}

impl Game {
    pub fn new() -> Game {
        Self {
            map: Map::new(),
            exit: false,
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
            area: Rect::new(0, 0, SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16),
        }
    }
}

impl Widget for &Game {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from("Dungeon Rustlers".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let canvas = Canvas::default()
            .x_bounds([0.0, (SCREEN_WIDTH - 1) as f64])
            .y_bounds([0.0, (SCREEN_HEIGHT - 1) as f64])
            .paint(|ctx| {
                for y in 0..SCREEN_HEIGHT {
                    for x in 0..SCREEN_WIDTH {
                        ctx.draw(&Rectangle {
                            x: x as f64,
                            y: y as f64,
                            width: 1.0,
                            height: 1.0,
                            color: Color::Red,
                        });
                    }
                }
            });
    }
}

//impl GameState for State {
//    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
//        self.map.render(ctx);
//        self.player.render(ctx);
//    }
//}
