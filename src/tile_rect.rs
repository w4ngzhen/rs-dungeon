use ggez::mint::Point2;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct TileRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl TileRect {
    pub fn left_top(&self) -> Point2<u32> {
        Point2::from([self.x, self.y])
    }
    pub fn right_bottom(&self) -> Point2<u32> {
        Point2::from([self.x + self.w, self.y + self.h])
    }

    pub fn intersect(&self, other: &TileRect) -> bool {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = x1 + self.w;
        let y2 = y1 + self.h;
        let other_x1 = other.x;
        let other_y1 = other.y;
        let other_x2 = other_x1 + other.w;
        let other_y2 = other_y1 + other.h;
        x1 <= other_x2 && x2 >= other_x1 && y1 <= other_y2 && y2 >= other_y1
    }

    pub fn center(&self) -> Point2<u32> {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = x1 + self.w;
        let y2 = y1 + self.h;
        Point2::from([(x1 + x2) / 2, (y1 + y2) / 2])
    }
}