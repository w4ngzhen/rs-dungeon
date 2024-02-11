use ggez::mint::Point2;
use crate::constants::TILE_WIDTH;

pub fn xy_idx(x: u64, y: u64) -> usize {
    (y as usize * TILE_WIDTH as usize) + x as usize
}

pub fn to_tuple<T>(p: Point2<T>) -> (T, T) {
    (p.x, p.y)
}