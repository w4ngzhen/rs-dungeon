use std::cmp::{max, min};
use doryen_fov::{FovAlgorithm, FovRecursiveShadowCasting, MapData};
use ggez::mint::Point2;
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
            viewshed.visible_tiles = calc_visibility(
                Point2::from([pos.x, pos.y]),
                viewshed.range,
                &map,
            );
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

fn calc_visibility(role_pos: Point2<u32>, range: u32, map: &Map) -> Vec<Point2<u32>> {
    let map_w = map.width;
    let map_h = map.height;
    let x_range = (max(0, role_pos.x as i32 - range as i32) as u32, min(role_pos.x + range, map_w));
    let y_range = (max(0, role_pos.y as i32 - range as i32) as u32, min(role_pos.y + range, map_h));
    let view_rect_w = x_range.1 - x_range.0;
    let view_rect_h = y_range.1 - y_range.0;
    let mut view_map_data = MapData::new(view_rect_w as usize, view_rect_h as usize);
    for origin_x in x_range.0..x_range.1 {
        for origin_y in y_range.0..y_range.1 {
            let origin_idx = xy_idx(origin_x, origin_y);
            if map.is_opaque(origin_idx) {
                let offset_x = origin_x - x_range.0;
                let offset_y = origin_y - y_range.0;
                view_map_data.set_transparent(offset_x as usize, offset_y as usize, false);
            }
        }
    }
    let mut fov = FovRecursiveShadowCasting::new();
    let role_offset_x = role_pos.x - x_range.0;
    let role_offset_y = role_pos.y - y_range.0;
    fov.compute_fov(&mut view_map_data, role_offset_x as usize, role_offset_y as usize, range as usize, true);
    let mut visible_points: Vec<Point2<u32>> = Vec::new();
    for origin_x in x_range.0..x_range.1 {
        for origin_y in y_range.0..y_range.1 {
            let offset_x = (origin_x - x_range.0) as usize;
            let offset_y = (origin_y - y_range.0) as usize;
            if view_map_data.is_in_fov(offset_x, offset_y) {
                visible_points.push(Point2::from([origin_x, origin_y]));
            }
        }
    }
    visible_points
}