use ratatui::{
    style::Color,
    widgets::canvas::{Context, Rectangle},
};

use crate::{
    core::traits::Drawable,
    shared::{Point, TileType, NUM_TILES, SCREEN_HEIGHT, SCREEN_WIDTH},
};

use super::Map;

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: vec![TileType::Flor; NUM_TILES],
        }
    }

    pub fn map_idx(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }

    //pub fn render(&self, ctx: &mut BTerm) {
    //    for y in 0..SCREEN_HEIGHT {
    //        for x in 0..SCREEN_WIDTH {
    //            let idx = Self::map_idx(x, y);
    //            match self.tiles[idx] {
    //                TileType::Flor => {
    //                    ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
    //                }
    //                TileType::Wall => {
    //                    ctx.set(x, y, GREEN, BLACK, to_cp437('.'));
    //                }
    //            }
    //        }
    //    }
    //}

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[Self::map_idx(point.x, point.y)] == TileType::Flor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        match self.in_bounds(point) {
            true => Some(Self::map_idx(point.x, point.y)),
            false => None,
        }
    }
}

impl Drawable for Map {
    //fn draw(&self, ctx: &mut Context) {
    //    for y in 0..SCREEN_HEIGHT {
    //        for x in 0..SCREEN_WIDTH {
    //            let idx = Self::map_idx(x, y);
    //            match self.tiles[idx] {
    //                TileType::Wall => ctx.draw(&Rectangle {
    //                    x: x as f64,
    //                    y: y as f64,
    //                    width: 1.0,
    //                    height: 1.0,
    //                    color: Color::Red,
    //                }),
    //                TileType::Flor => ctx.draw(&Rectangle {
    //                    x: x as f64,
    //                    y: y as f64,
    //                    width: 1.0,
    //                    height: 1.0,
    //                    color: Color::Green,
    //                }),
    //            }
    //        }
    //    }
    //}
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tiles: vec![TileType::Flor; NUM_TILES],
        }
    }
}
