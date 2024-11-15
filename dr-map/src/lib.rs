const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 50;
const NUM_TILES: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Flor,
}

pub struct Map {
    tiles: Vec<TileType>,
}

mod map;
