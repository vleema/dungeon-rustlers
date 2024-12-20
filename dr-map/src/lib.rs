pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Flor,
}

pub struct Map {
    tiles: Vec<TileType>,
}

mod map;
