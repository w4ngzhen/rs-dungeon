use crate::constants::TILE_WIDTH;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * TILE_WIDTH as usize) + x as usize
}

pub fn calc_tile_size(pix_w: u32, pix_h: u32, tile_w: u32, tile_h: u32) -> (u32, u32) {
    (pix_w / tile_w, pix_h  / tile_h)
}