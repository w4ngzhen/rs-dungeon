use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::framework::constants::TILE_SIZE;
use crate::framework::game_state::GameState;

pub struct GameWindow {
    tile_w: u32,
    tile_h: u32,
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

impl GameWindow {
    pub fn new(tile_w: u32, tile_h: u32) -> Result<GameWindow, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let pix_w = tile_w * TILE_SIZE;
        let pix_h = tile_h * TILE_SIZE;
        let window = video_subsystem
            .window("rust-sdl2 demo: Events", pix_w, pix_h)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(GameWindow {
            tile_w,
            tile_h,
            sdl_context,
            canvas,
        })
    }

    pub fn main_loop(&mut self, gs: &mut dyn GameState) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    // skip mouse motion intentionally because of the verbose it might cause.
                    Event::MouseMotion { .. } => {}
                    e => {
                        println!("{:?}", e);
                    }
                }
            }
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            // The rest of the game loop goes here...
        }
    }
}