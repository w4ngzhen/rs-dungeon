#[derive(Copy, Clone, Debug)]
pub struct TilePos {
    pub tile_x: u64,
    pub tile_y: u64,
}

impl TilePos {
    pub fn new(tile_x: u64,
               tile_y: u64, ) -> Self {
        TilePos {
            tile_x,
            tile_y,
        }
    }
}