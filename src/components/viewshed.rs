use ggez::mint::Point2;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point2<u64>>,
    pub range: u64,
    /// flag indicate the field of view changed.
    pub invalid: bool,
}