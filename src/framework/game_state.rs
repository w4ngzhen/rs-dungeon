use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub trait GameState {
    fn next_tick(&mut self, tick_context: &mut TickContext) -> Option<i32>;
}

pub struct TickContext<'a> {
    pub event: &'a Option<Event>,
    pub canvas: &'a mut WindowCanvas,
    pub tile_size_w: u32,
    pub tile_size_h: u32,
}

impl<'a> TickContext<'a> {
    pub fn draw(&mut self, tile_x: i32, tile_y: i32, tile_code: &str) {
        let rect = self.calc_tile_rect(tile_x, tile_y);
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn calc_tile_rect(&self, tile_x: i32, tile_y: i32) -> Rect {
        let pix_x = tile_x * self.tile_size_w as i32;
        let pix_y = tile_y * self.tile_size_h as i32;
        Rect::new(pix_x, pix_y, self.tile_size_w, self.tile_size_h)
    }
}