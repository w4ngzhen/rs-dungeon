use std::cmp::{max, min};
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rect};
use crate::constants::tile::TileType;
use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::utils::xy_idx;

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub visited_tiles: Vec<bool>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new_map() -> Self {
        const MAP_SIZE: u32 = TILE_WIDTH * TILE_HEIGHT;

        let mut map_tiles = vec![TileType::Wall; MAP_SIZE as usize];

        let mut rooms: Vec<Rect> = Vec::new();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: u32 = 6;
        const MAX_SIZE: u32 = 10;

        let mut rng = RandomNumberGenerator::seeded(123);

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, (TILE_WIDTH - w - 1) as i32) - 1;
            let y = rng.roll_dice(1, (TILE_HEIGHT - h - 1) as i32) - 1;
            let new_room = Rect::with_size(x, y, w as i32, h as i32);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                apply_room_to_map(&new_room, &mut map_tiles);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center().to_tuple();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center().to_tuple();
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
            width: TILE_WIDTH as i32,
            height: TILE_HEIGHT as i32,
        };
        map
    }
}

impl BaseMap for Map {
    /// it will used by get field of view.
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (TILE_WIDTH * TILE_HEIGHT) as usize {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (TILE_WIDTH * TILE_HEIGHT) as usize {
            map[idx] = TileType::Floor;
        }
    }
}