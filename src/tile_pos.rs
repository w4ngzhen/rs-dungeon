#[derive(Copy, Clone)]
pub struct TilePos {
    pub tile_x: u32,
    pub tile_y: u32,
}

impl TilePos {
    pub fn new(tile_x: u32,
               tile_y: u32, ) -> Self {
        TilePos {
            tile_x,
            tile_y,
        }
    }
}