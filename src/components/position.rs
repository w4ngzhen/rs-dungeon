use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}

impl Position {
    pub fn new(x: u64, y: u64) -> Self {
        Self {
            x,
            y,
        }
    }

    #[allow(unused)]
    pub fn from(data: [u64; 2]) -> Self {
        let [x, y] = data;
        Position::new(x, y)
    }

    pub fn to_tuple(&self) -> (u64, u64) {
        (self.x, self.y)
    }
}