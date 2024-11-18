use crate::shared::Point;

use super::Player;

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn update(&mut self, x: i32, y: i32) {
        self.position.x += x;
        self.position.y += y;
    }
}
