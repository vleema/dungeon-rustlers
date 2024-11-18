use dungeon_rustlers::core::game::Game;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = Game::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
