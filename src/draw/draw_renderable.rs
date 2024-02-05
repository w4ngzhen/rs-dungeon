use crate::components::position::Position;
use crate::components::renderable::Renderable;
use crate::game_context::GameContext;
use crate::tile_pos::TilePos;

pub fn draw_renderable(pos: &Position, renderable: &Renderable, ctx: &mut GameContext) {
    ctx.draw_tile_block(&TilePos::new(pos.x as u32, pos.y as u32), renderable.fg);
}