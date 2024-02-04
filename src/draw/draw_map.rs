use rltk::{RGB, Rltk};
use specs::World;
use crate::constants::tile::TileType;
use crate::constants::TILE_WIDTH;
use crate::game_tick_ctx::GameTickCtx;
use crate::map::Map;

pub fn draw_map(ecs: &World, ctx: &mut GameTickCtx) {
    let map = ecs.fetch::<Map>();
    for (map_idx, tile_type) in map.tiles.iter().enumerate() {
        let x = (map_idx % TILE_WIDTH as usize) as i32;
        let y = (map_idx / TILE_WIDTH as usize) as i32;

        let fb_color = if map.revealed_tiles[map_idx] {
            RGB::from_f32(0.0, 1.0, 0.0)
        } else if map.visited_tiles[map_idx] {
            RGB::from_f32(0.6, 0.6, 0.6)
        } else {
            RGB::from_f32(0.2, 0.2, 0.2)
        };
        match tile_type {
            TileType::Floor => {
                ctx.draw_text(x, y, ".");
            }
            TileType::Wall => {
                ctx.draw_text(x, y, "#");
            }
        }
    }
}