use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}