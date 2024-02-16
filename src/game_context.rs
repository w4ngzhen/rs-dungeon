use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::{Canvas, Color, DrawParam, PxScale, Rect, Text, TextAlign, TextFragment, TextLayout};
use ggez::mint::Point2;
use crate::components::position::Position;

pub struct GameContext<'a> {
    pub canvas: &'a mut Canvas,
    pub tile_size_w: f32,
    pub tile_size_h: f32,
}

impl<'a> GameContext<'a> {
    pub fn draw_tile_block(&mut self, tile_pos: &Position, color: Color) {
        let rect = self.tile_pix_rect(tile_pos);
        self.canvas.draw(&graphics::Quad,
                         DrawParam::new()
                             .dest(rect.point())
                             .scale(rect.size())
                             .color(color))
    }

    pub fn draw_tile_text(&mut self, tile_pos: &Position, fg_color: Color, bg_color: Option<Color>, text: &str) {
        let rect = self.tile_pix_rect(tile_pos);
        // build Text data.
        let mut text_data = Text::new(TextFragment {
            text: text.to_string(),
            color: Some(fg_color),
            ..Default::default()
        });
        text_data
            .set_bounds(Vec2::new(rect.w, rect.h))
            .set_scale(PxScale { x: rect.w, y: rect.h })
            .set_layout(
                TextLayout {
                    v_align: TextAlign::Middle,
                    h_align: TextAlign::Middle,
                }
            );
        if let Some(bg) = bg_color {
            self.draw_tile_block(tile_pos, bg);
        }
        self.canvas.draw(
            &text_data,
            DrawParam::default().dest(rect.center()),
        );
    }

    fn tile_pix_pos(&self, tile_pos: &Position) -> Point2<f32> {
        let pix_x = tile_pos.x as f32 * self.tile_size_w;
        let pix_y = tile_pos.y as f32 * self.tile_size_h;
        Point2::from([pix_x, pix_y])
    }

    fn tile_pix_rect(&self, tile_pos: &Position) -> Rect {
        let pos = self.tile_pix_pos(tile_pos);
        Rect::new(pos.x, pos.y, self.tile_size_w, self.tile_size_h)
    }
}