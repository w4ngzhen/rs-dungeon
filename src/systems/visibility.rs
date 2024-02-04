use rltk::{field_of_view, Point};
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::viewshed::Viewshed;
use crate::map::Map;
use crate::utils::xy_idx;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (WriteExpect<'a, Map>,
                       Entities<'a>,
                       WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Position>,
                       ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed_store, pos_store, player_store) = data;
        for (viewshed, pos, ent) in (&mut viewshed_store, &pos_store, &entities).join() {
            if !viewshed.invalid {
                continue;
            }
            // only recalculate when viewshed is invalid.
            // every entity with viewshed comp should update their viewshed data;
            viewshed.invalid = true;
            viewshed.visible_tiles.clear();
            // use utils to calculate field of view.
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
            // but only Player can reveal the map.
            let opt_p: Option<&Player> = player_store.get(ent);
            if let Some(_p) = opt_p {
                for val in map.revealed_tiles.iter_mut() {
                    *val = false;
                }
                for vis in viewshed.visible_tiles.iter() {
                    let visible_idx = xy_idx(vis.x, vis.y);
                    // remember visited region.
                    map.visited_tiles[visible_idx] = true;
                    // set current reveal tiles.
                    map.revealed_tiles[visible_idx] = true;
                }
            }
        }
    }
}