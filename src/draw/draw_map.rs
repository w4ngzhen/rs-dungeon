use ggez::graphics::Color;
use specs::World;
use crate::constants::tile::TileType;
use crate::constants::TILE_WIDTH;
use crate::game_context::GameContext;
use crate::map::Map;
use crate::tile_pos::TilePos;

pub fn draw_map(ecs: &World, ctx: &mut GameContext) {
    let map = ecs.fetch::<Map>();
    for (map_idx, tile_type) in map.tiles.iter().enumerate() {
        let x = (map_idx % TILE_WIDTH as usize) as u32;
        let y = (map_idx / TILE_WIDTH as usize) as u32;
        let tile_pos = TilePos::new(x, y);

        let fg_color = if map.revealed_tiles[map_idx] {
            Color::from_rgb(0, 255, 0)
        } else if map.visited_tiles[map_idx] {
            Color::from_rgb(120, 120, 120)
        } else {
            Color::from_rgb(50, 50, 50)
        };
        let (char, _tile_code) = match tile_type {
            TileType::Floor => {
                (".", 0)
            }
            TileType::Wall => {
                ("#", 1)
            }
        };
        ctx.draw_tile_text(&tile_pos, fg_color, None, char);
    }
}