use sdl2::pixels::Color;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub c: char,
    pub fg: Color,
    pub bg: Color,
}