use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub trait GameState {
    fn next_tick(&mut self, tick_context: &mut TickContext) -> Option<i32>;
}

pub struct TickContext<'a> {
    pub event: &'a Option<Event>,
    pub canvas: &'a mut WindowCanvas,
    pub font: &'a Font<'a, 'a>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub tile_size_w: u32,
    pub tile_size_h: u32,
}

impl<'a> TickContext<'a> {
    pub fn draw(&mut self, tile_x: i32, tile_y: i32, tile_code: &str) {
        let rect = self.calc_tile_rect(tile_x, tile_y);
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn draw_text(&mut self, tile_x: i32, tile_y: i32, text: &str) {
        let surface = self.font
            .render(text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .unwrap();
        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let (pix_x, pix_y) = self.calc_tile_pos(tile_x, tile_y);
        self.canvas.copy(&texture, None, Some(Rect::new(pix_x, pix_y, width, height))).unwrap();
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn calc_tile_pos(&self, tile_x: i32, tile_y: i32) -> (i32, i32) {
        let pix_x = tile_x * self.tile_size_w as i32;
        let pix_y = tile_y * self.tile_size_h as i32;
        (pix_x, pix_y)
    }

    fn calc_tile_rect(&self, tile_x: i32, tile_y: i32) -> Rect {
        let (pix_x, pix_y) = self.calc_tile_pos(tile_x, tile_y);
        Rect::new(pix_x, pix_y, self.tile_size_w, self.tile_size_h)
    }
}