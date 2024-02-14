use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
#[derive(Clone, Debug)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}