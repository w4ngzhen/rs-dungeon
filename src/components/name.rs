use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Name {
    pub name: String,
}