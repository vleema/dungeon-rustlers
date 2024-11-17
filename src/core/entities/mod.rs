use crate::shared::{Point, TileType};

#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<TileType>,
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub position: Point,
}

pub mod map;
pub mod player;
