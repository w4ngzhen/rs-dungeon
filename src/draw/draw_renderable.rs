use crate::components::position::Position;
use crate::components::renderable::Renderable;
use crate::game_context::GameContext;

pub fn draw_renderable(pos: &Position, renderable: &Renderable, ctx: &mut GameContext) {
    // ctx.draw_tile_block(&pos, renderable.fg);
    ctx.draw_tile_text(&pos, renderable.fg, None, renderable.c.to_string().as_str());
}