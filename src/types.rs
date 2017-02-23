use std::ops;

pub type Coord = i16;
pub type Dimension = u32;

pub type Triangle = [Point; 3];

macro_rules! trigon {
    ( $p1:expr, $p2:expr, $p3:expr ) => { [$p1, $p2, $p3] }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}

macro_rules! pt {
    ( $x:expr, $y:expr ) => { Point { x: $x, y: $y }}
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        pt!(self.x + other.x, self.y + other.y)
    }
}
