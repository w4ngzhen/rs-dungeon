use crate::components::position::Position;
use crate::components::renderable::Renderable;
use crate::game_context::GameContext;
use crate::tile_pos::TilePos;

pub fn draw_renderable(pos: &Position, renderable: &Renderable, ctx: &mut GameContext) {
    let pos = TilePos::new(pos.x as u32, pos.y as u32);
    // ctx.draw_tile_block(&pos, renderable.fg);
    ctx.draw_tile_text(&pos, renderable.fg, None, renderable.c.to_string().as_str());
}