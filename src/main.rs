use std::env;
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
        ctx.draw_text(self.x, self.y, "hello, world.");
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
    // init SDL
    let sdl_context = sdl2::init()?;
    // init SDL TTF
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = env::var("RS_GAME_FONT_PATH").map_err(|e| format!("{}: {}", e.to_string(), "RS_GAME_FONT_PATH"))?;
    let font = ttf_context.load_font(font_path, 14)?;
    // todo init SDL Mix
    // build GameWindow
    let mut window = GameWindow::new(80, 50, &sdl_context, &font)?;
    let mut gs = MyGameState { x: 0, y: 0 };
    window.main_loop(&mut gs);
    Ok(())
}
