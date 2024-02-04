use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    /// flag indicate the field of view changed.
    pub invalid: bool,
}