use std::ops;

pub type Coord = f64;
pub type PixCoord = i16;
pub type Dimension = u32;


#[derive(Clone, Copy)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

macro_rules! trigon {
    ( $p1:expr, $p2:expr, $p3:expr ) => { Triangle::new($p1, $p2, $p3) }
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        Triangle { p1:p1, p2:p2, p3:p3 }
    }

    pub fn to_tuple(self) -> (Point, Point, Point) {
        (self.p1, self.p2, self.p3)
    }

    pub fn to_arr(self) -> [Point; 3] {
        [self.p1, self.p2, self.p3]
    }

    pub fn normal(self) -> Point {
        let d1 = self.p2 - self.p1;
        let d2 = self.p3 - self.p1;
        d1.cross(d2).normalized()
    }
}

impl ops::Mul<Transform> for Triangle {
    type Output = Triangle;

    fn mul(self, trans: Transform) -> Triangle {
        trigon![
            self.p1 * trans,
            self.p2 * trans,
            self.p3 * trans
        ]
    }
}


const DIM: usize = 3;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
    pub z: Coord,
}

macro_rules! pt_2d {
    ( $x:expr, $y:expr ) => { pt![$x, $y, 0.] }
}

macro_rules! pt {
    ( $x:expr, $y:expr, $z:expr ) => { Point { x: $x, y: $y, z: $z }}
}

impl Point {
    pub fn from_vec(mut v: Vec<Coord>) -> Point {
        while v.len() < DIM { v.push(0.); }
        if v.len() < DIM + 1 { v.push(1.); }
        Point {
            x: v[0] / v[DIM],
            y: v[1] / v[DIM],
            z: v[2] / v[DIM]
        }
    }

    pub fn from_array(arr: [f64; DIM + 1]) -> Point {
        Point {
            x: arr[0] / arr[DIM],
            y: arr[1] / arr[DIM],
            z: arr[2] / arr[DIM]
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

    pub fn dot(self, other: Point) -> f64 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    pub fn cross(self, other: Point) -> Point {
        pt![
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        ]
    }

    pub fn magnitude(self) -> f64 {
        (
            self.x * self.x +
            self.y * self.y +
            self.z * self.z
        ).sqrt()
    }

    pub fn normalized(self) -> Point {
        self * (1. / self.magnitude())
    }
}

// Scaling
impl ops::Mul<Coord> for Point {
    type Output = Point;

    fn mul(self, other: Coord) -> Point {
        pt!(
            self.x * other,
            self.y * other,
            self.z * other
        )
    }
}

// Point addition
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

// Point subtraction
impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        pt!(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z
        )
    }
}

// Matrix transformation
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

#[allow(dead_code)]
impl Transform {
    pub fn identity() -> Transform {
        let mut data = [[0.0; DIM + 1]; DIM + 1];
        for i in 0 .. DIM + 1 { data[i][i] = 1.0 }
        Transform { data: data }
    }

    pub fn translate(off: Point) -> Transform {
        let mut t = Transform::identity();
        let arr_in = off.to_array();
        for i in 0 .. DIM { t.data[i][DIM] = arr_in[i] as f64 }
        t
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

    pub fn scale(x: f64, y: f64, z: f64) -> Transform {
        let mut data = [[0.; DIM + 1]; DIM + 1];
        data[0][0] = x;
        data[1][1] = y;
        data[2][2] = z;
        data[DIM][DIM] = 1.;
        Transform { data: data }
    }

    pub fn perspective() -> Transform {
        let mut t = Transform::identity();
        t.data[DIM    ][DIM    ] =  0.;
        t.data[DIM    ][DIM - 1] = -1.;
        t.data[DIM - 1][DIM - 1] = -1.;
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
