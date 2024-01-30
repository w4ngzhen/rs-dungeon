use crate::framework::game_state::{GameState, TickContext};
use crate::framework::game_window::GameWindow;

mod framework;

struct MyGameState {}

impl GameState for MyGameState {
    fn next_tick(&mut self, _ctx: &TickContext) -> () {
        todo!()
    }
}

fn main() -> Result<(), String> {
    let mut window = GameWindow::new(80, 50)?;
    let mut gs = MyGameState {};
    window.main_loop(&mut gs);
    Ok(())
}
