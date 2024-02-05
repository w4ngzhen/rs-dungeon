use ggez::graphics;
use ggez::graphics::{Canvas, Color, Rect};
use ggez::mint::Point2;
use crate::tile_pos::TilePos;

pub struct GameContext<'a> {
    // pub ctx: &'a Context,
    pub canvas: &'a mut Canvas,
    pub tile_size_w: u32,
    pub tile_size_h: u32,
}

impl<'a> GameContext<'a> {
    pub fn draw_tile_block(&mut self, tile_pos: &TilePos, color: Color) {
        let rect = self.tile_pix_rect(tile_pos);
        self.canvas.draw(&graphics::Quad,
                         graphics::DrawParam::new()
                             .dest(rect.point())
                             .scale(rect.size())
                             .color(color))
    }

    fn tile_pix_pos(&self, tile_pos: &TilePos) -> Point2<f32> {
        let pix_x = tile_pos.tile_x * self.tile_size_w;
        let pix_y = tile_pos.tile_y * self.tile_size_h;
        Point2::from([pix_x as f32, pix_y as f32])
    }

    fn tile_pix_rect(&self, tile_pos: &TilePos) -> Rect {
        let pos = self.tile_pix_pos(tile_pos);
        Rect::new(pos.x, pos.y, self.tile_size_w as f32, self.tile_size_h as f32)
    }
}