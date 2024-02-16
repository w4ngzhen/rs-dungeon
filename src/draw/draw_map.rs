use ggez::graphics::Color;
use specs::World;
use crate::components::position::Position;
use crate::constants::tile::TileType;
use crate::constants::TILE_WIDTH;
use crate::game_context::GameContext;
use crate::map::Map;

pub fn draw_map(ecs: &World, ctx: &mut GameContext) {
    let map = ecs.fetch::<Map>();
    for (map_idx, tile_type) in map.tiles.iter().enumerate() {
        let x = map_idx as u64 % TILE_WIDTH;
        let y = map_idx as u64 / TILE_WIDTH;
        let tile_pos = Position::new(x, y);

        let fg_color = if map.revealed_tiles[map_idx] {
            Some(Color::from_rgb(0, 255, 0))
        } else if map.visited_tiles[map_idx] {
            Some(Color::from_rgb(120, 120, 120))
        } else {
            None
        };
        if fg_color == None {
            continue
        }
        let (char, _tile_code) = match tile_type {
            TileType::Floor => {
                (".", 0)
            }
            TileType::Wall => {
                ("#", 1)
            }
        };
        ctx.draw_tile_text(&tile_pos, fg_color.unwrap() , None, char);
    }
}