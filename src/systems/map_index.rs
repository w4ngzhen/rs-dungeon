use specs::{Join, ReadStorage, System, WriteExpect};
use crate::components::blocked_tile::BlockedTile;
use crate::components::position::Position;
use crate::constants::tile::TileType;
use crate::map::Map;
use crate::utils::xy_idx;

pub struct MapIndexSystem {}

impl<'a> System<'a> for MapIndexSystem {
    type SystemData = (WriteExpect<'a, Map>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, BlockedTile>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position_store, blocked_tile_store) = data;
        // wall
        for x_idx in 0..map.width {
            for y_idx in 0..map.height {
                let idx = xy_idx(x_idx, y_idx);
                if map.tiles[idx] == TileType::Wall {
                    map.blocked_tiles[idx] = true;
                }
            }
        }
        // blocked component.
        for (pos, _) in (&position_store, &blocked_tile_store).join() {
            println!("block position: {:?}", pos);
            let idx = xy_idx(pos.x, pos.y);
            map.blocked_tiles[idx] = true;
        }
    }
}
