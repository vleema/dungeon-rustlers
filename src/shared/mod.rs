pub const SCREEN_WIDTH: i32 = 120;
pub const SCREEN_HEIGHT: i32 = 30;
pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 80, y: 50 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Flor,
}
