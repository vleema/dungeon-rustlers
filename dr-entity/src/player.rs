use bracket_lib::{
    color::{BLACK, WHITE},
    prelude::{to_cp437, BTerm, Point},
};

use crate::Player;

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
}
