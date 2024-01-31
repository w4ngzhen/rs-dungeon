use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::framework::game_state::{GameState, TickContext};
use crate::framework::game_window::GameWindow;

mod framework;

struct MyGameState {
    x: i32,
    y: i32,
}

impl GameState for MyGameState {
    fn next_tick(&mut self, ctx: &mut TickContext) -> Option<i32> {
        // handle event.
        if let Some(event) = ctx.event {
            // handle input.
            match event {
                Event::Quit { .. } => {
                    return Some(-1);
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                    let (dx, dy) = calc_move_delta(key);
                    self.x += dx;
                    self.y += dy;
                }
                _ => {
                    // nothing now
                }
            }
        }
        // run system.
        // todo
        // draw.
        ctx.clear(Color::RGB(255, 255, 255));
        ctx.draw(self.x, self.y, "");
        Some(0)
    }
}

fn calc_move_delta(key: &Keycode) -> (i32, i32) {
    match key {
        Keycode::Up => (0, -1),
        Keycode::Down => (0, 1),
        Keycode::Left => (-1, 0),
        Keycode::Right => (1, 0),
        _ => (0, 0)
    }
}

fn main() -> Result<(), String> {
    let mut window = GameWindow::new(80, 50)?;
    let mut gs = MyGameState { x: 0, y: 0 };
    window.main_loop(&mut gs);
    Ok(())
}
