use std::ops;

pub type Coord = i16;
pub type Dimension = u32;

pub type Triangle = [Point; 3];

macro_rules! trigon {
    ( $p1:expr, $p2:expr, $p3:expr ) => { [$p1, $p2, $p3] }
}

const DIM: usize = 3;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
    pub z: Coord,
}

impl Point {
    pub fn from_array(arr: [f64; DIM + 1]) -> Point {
        Point {
            x: (arr[0] / arr[DIM]) as Coord,
            y: (arr[1] / arr[DIM]) as Coord,
            z: (arr[2] / arr[DIM]) as Coord
        }
    }

    pub fn to_array(self) -> [f64; DIM + 1] {
        [
            self.x as f64,
            self.y as f64,
            self.z as f64,
            1.
        ]
    }
}

macro_rules! pt_2d {
    ( $x:expr, $y:expr ) => { pt![$x, $y, 0] }
}

macro_rules! pt {
    ( $x:expr, $y:expr, $z:expr ) => { Point { x: $x, y: $y, z: $z }}
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        pt!(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z
        )
    }
}

impl ops::Mul<Transform> for Point {
    type Output = Point;

    fn mul(self, rhs: Transform) -> Point {
        let arr_in = self.to_array();
        let mut arr_out = [0.; DIM + 1];
        for i in 0 .. DIM + 1 {
            arr_out[i] = rhs.data[i]
                .iter()
                .zip(arr_in.iter())
                .map(|(a, b)| a * b)
                .sum()
        }

        Point::from_array(arr_out)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    data: [[f64; DIM + 1]; DIM + 1]
}

impl Transform {
    pub fn identity() -> Transform {
        let mut data = [[0.0; DIM + 1]; DIM + 1];
        for i in 0 .. DIM + 1 { data[i][i] = 1.0 }
        Transform { data: data }
    }

    pub fn rotate_x(theta: f64) -> Transform {
        let mut t = Transform::identity();
        t.data[1][1] =  theta.cos();
        t.data[1][2] =  theta.sin();
        t.data[2][1] = -theta.sin();
        t.data[2][2] =  theta.cos();
        t
    }

    pub fn rotate_y(theta: f64) -> Transform {
        let mut t = Transform::identity();
        t.data[0][0] =  theta.cos();
        t.data[0][2] =  theta.sin();
        t.data[2][0] = -theta.sin();
        t.data[2][2] =  theta.cos();
        t
    }

    pub fn rotate_z(theta: f64) -> Transform {
        let mut t = Transform::identity();
        t.data[0][0] =  theta.cos();
        t.data[0][1] =  theta.sin();
        t.data[1][0] = -theta.sin();
        t.data[1][1] =  theta.cos();
        t
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
                    data[i][j] += self.data[i][k] * rhs.data[k][j]
                }
            }
        }

        Transform { data: data }
    }
}
