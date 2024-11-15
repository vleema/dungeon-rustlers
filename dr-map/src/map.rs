use bracket_lib::{
    color::{BLACK, GREEN, YELLOW},
    prelude::{to_cp437, BTerm},
};

use crate::{TileType, NUM_TILES, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::Map;

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: vec![TileType::Flor; NUM_TILES],
        }
    }

    pub fn map_idx(x: usize, y: usize) -> usize {
        (y * SCREEN_WIDTH) + x
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = Self::map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Flor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('.'));
                    }
                }
            }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tiles: vec![TileType::Flor; NUM_TILES],
        }
    }
}
