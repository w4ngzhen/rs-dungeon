use ggez::mint::Point2;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point2<u32>>,
    pub range: u32,
    /// flag indicate the field of view changed.
    pub invalid: bool,
}