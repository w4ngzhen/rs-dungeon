use std::cmp::{max, min};
use rltk::RandomNumberGenerator;
use crate::constants::tile::TileType;
use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::tile_rect::TileRect;
use crate::utils::{to_tuple, xy_idx};

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub visited_tiles: Vec<bool>,
    pub rooms: Vec<TileRect>,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn new_map() -> Self {
        const MAP_SIZE: u32 = TILE_WIDTH * TILE_HEIGHT;

        let mut map_tiles = vec![TileType::Wall; MAP_SIZE as usize];

        let mut rooms: Vec<TileRect> = Vec::new();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: u32 = 6;
        const MAX_SIZE: u32 = 10;

        let mut rng = RandomNumberGenerator::seeded(123);

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, (TILE_WIDTH - w - 1) as i32) - 1;
            let y = rng.roll_dice(1, (TILE_HEIGHT - h - 1) as i32) - 1;
            let new_room = TileRect {
                x: x as u32,
                y: y as u32,
                w,
                h,
            };
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                println!("new room: {:?}", new_room);
                apply_room_to_map(&new_room, &mut map_tiles);

                if !rooms.is_empty() {
                    let (new_x, new_y) = to_tuple(new_room.center());
                    let (prev_x, prev_y) = to_tuple(rooms[rooms.len() - 1].center());
                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(&mut map_tiles, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut map_tiles, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut map_tiles, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut map_tiles, prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }
        }

        let map = Map {
            tiles: map_tiles,
            revealed_tiles: vec![false; MAP_SIZE as usize],
            visited_tiles: vec![false; MAP_SIZE as usize],
            rooms,
            width: TILE_WIDTH,
            height: TILE_HEIGHT,
        };
        map
    }

    pub fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

fn apply_room_to_map(room: &TileRect, map: &mut [TileType]) {
    let lt = room.left_top();
    let rb = room.right_bottom();
    for y in lt.y + 1..=rb.y {
        for x in lt.x + 1..=rb.x {
            println!("floor: {}, {}", x, y);
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: u32, x2: u32, y: u32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (TILE_WIDTH * TILE_HEIGHT) as usize {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: u32, y2: u32, x: u32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (TILE_WIDTH * TILE_HEIGHT) as usize {
            map[idx] = TileType::Floor;
        }
    }
}