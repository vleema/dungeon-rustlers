use bracket_lib::prelude::{main_loop, BError, BTermBuilder, GameState};
use dr_map::Map;

const FPS: f32 = 10.0;

struct State {
    map: Map,
}

impl State {
    fn new() -> State {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Rustlers")
        .with_fps_cap(FPS)
        .build()?;

    main_loop(context, State::new())
}
