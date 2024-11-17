use bracket_lib::prelude::{main_loop, BError, BTermBuilder, GameState, Point};
use dr_entity::Player;
use dr_map::{Map, SCREEN_HEIGHT, SCREEN_WIDTH};

const FPS: f32 = 10.0;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> State {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.cls();
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Rustlers")
        .with_fps_cap(FPS)
        .build()?;

    main_loop(context, State::new())
}
