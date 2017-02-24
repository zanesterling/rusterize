use std::ops;

pub type Coord = i16;
pub type Dimension = u32;

pub type Triangle = [Point; 3];

macro_rules! trigon {
    ( $p1:expr, $p2:expr, $p3:expr ) => { [$p1, $p2, $p3] }
}

const DIM: usize = 2;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}

impl Point {
    pub fn from_array(arr: [Coord; DIM + 1]) -> Point {
        return Point { x: arr[0], y: arr[1] }
    }

    pub fn to_array(self) -> [Coord; DIM + 1] {
        [self.x, self.y, 1]
    }
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

impl ops::Mul<Transform> for Point {
    type Output = Point;

    fn mul(self, rhs: Transform) -> Point {
        let arr_in = self.to_array();
        let mut arr_out = [0; DIM + 1];
        for i in 0 .. DIM + 1 {
            arr_out[i] = rhs.data[i]
                .iter()
                .zip(arr_in.iter())
                .map(|(a, b)| a * (*b as f64))
                .fold(0.0, |a, b| a + b) as Coord;
        }

        Point::from_array(arr_out)
    }
}

#[derive(Clone, Copy)]
pub struct Transform {
    data: [[f64; DIM + 1]; DIM + 1]
}

impl Transform {
    pub fn identity() -> Transform {
        let mut data = [[0.0; DIM + 1]; DIM + 1];
        for i in 0 .. DIM { data[i][i] = 1.0 }
        Transform { data: data }
    }

    pub fn rotate(theta: f64) -> Transform {
        let mut data = [[0.0; DIM + 1]; DIM + 1];
        if DIM == 2 {
            data[0][0] =  theta.cos();
            data[0][1] =  theta.sin();
            data[1][0] = -theta.cos();
            data[1][1] =  theta.cos();
        }

        data[DIM][DIM] = 1.0;
        Transform { data: data }
    }

    pub fn translate(off: Point) -> Transform {
        let mut t = Transform::identity();
        let arr_in = off.to_array();
        for i in 0 .. DIM { t.data[i][DIM] = arr_in[i] as f64 }
        t
    }
}

impl ops::Mul for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Transform {
        let mut data = [[0.0; DIM + 1]; DIM + 1];

        for i in 0 .. DIM + 1 {
            for j in 0 .. DIM + 1 {
                for k in 0 .. DIM + 1 {
                    data[i][j] += self.data[i][k] + rhs.data[k][j]
                }
            }
        }

        Transform { data: data }
    }
}
