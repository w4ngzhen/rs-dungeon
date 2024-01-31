use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::framework::constants::TILE_SIZE;

pub trait GameState {
    fn next_tick(&mut self, tick_context: &mut TickContext) -> Option<i32>;
}

pub struct TickContext<'a> {
    pub event: &'a Option<Event>,
    pub canvas: &'a mut WindowCanvas,
}

impl<'a> TickContext<'a> {
    pub fn draw(&mut self, tile_x: i32, tile_y: i32, tile_code: &str) {
        let rect = TickContext::calc_tile_rect(tile_x, tile_y);
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn calc_tile_rect(tile_x: i32, tile_y: i32) -> Rect {
        let pix_x = tile_x * TILE_SIZE as i32;
        let pix_y = tile_y * TILE_SIZE as i32;
        Rect::new(pix_x, pix_y, TILE_SIZE, TILE_SIZE)
    }
}