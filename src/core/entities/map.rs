use crate::shared::{Point, TileType, NUM_TILES, SCREEN_HEIGHT, SCREEN_WIDTH};

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

    pub fn equals(p1: Point, p2: Point) -> bool {
        Self::map_idx(p1.x, p1.y) == Self::map_idx(p2.x, p2.y)
    }

    pub fn get_tile(&self, idx: usize) -> TileType {
        self.tiles[idx]
    }

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

impl Default for Map {
    fn default() -> Self {
        Self {
            tiles: vec![TileType::Flor; NUM_TILES],
        }
    }
}
