use crate::constants::TILE_WIDTH;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * TILE_WIDTH as usize) + x as usize
}