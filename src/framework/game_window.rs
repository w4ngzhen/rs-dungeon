use std::time::Duration;
use sdl2::event::{Event, WindowEvent};
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::framework::game_state::{GameState, TickContext};

pub struct GameWindow {
    tile_w: u32,
    tile_h: u32,
    tile_size_w: u32,
    tile_size_h: u32,
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

impl GameWindow {
    pub fn new(tile_w: u32, tile_h: u32) -> Result<GameWindow, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let pix_w = 800;
        let pix_h = 600;

        let (tile_size_w, tile_size_h) = GameWindow::calc_tile_size(pix_w, pix_h, tile_w, tile_h);

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
            tile_size_w,
            tile_size_h,
            sdl_context,
            canvas,
        })
    }

    pub(crate) fn main_loop(&mut self, gs: &mut dyn GameState) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            let event_opt = event_pump.poll_event();
            let (tile_size_w, tile_size_h) = match event_opt {
                Some(Event::Window { win_event: WindowEvent::Resized(w, h), .. }) => {
                    let tile_size = GameWindow::calc_tile_size(w as u32, h as u32, self.tile_w, self.tile_h);
                    self.tile_size_w = tile_size.0;
                    self.tile_size_h = tile_size.1;
                    tile_size
                }
                _ => (self.tile_size_w, self.tile_size_h)
            };
            let mut tick_ctx = TickContext {
                event: &event_opt,
                canvas: &mut self.canvas,
                tile_size_w,
                tile_size_h,
            };
            if let Some(-1) = gs.next_tick(&mut tick_ctx) {
                break 'running;
            }
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            // The rest of the game loop goes here...
        }
    }

    pub fn calc_tile_size(pix_w: u32, pix_h: u32, tile_w: u32, tile_h: u32) -> (u32, u32) {
        (pix_w / tile_w, pix_h / tile_h)
    }
}