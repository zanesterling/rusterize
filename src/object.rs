use std::error;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

use consts;
use renderer::Renderer;
use screen::Screen;
use types::*;

// FIXME: Add transformation cacheing.
pub struct Object {
    translation: Transform,
    rotation:    Transform,
    scaling:     Transform,
    triangles:   Vec<Triangle>,
}

#[allow(dead_code)]
impl Object {
    pub fn new(tris: Vec<Triangle>) -> Object {
        Object {
            translation: Transform::identity(),
            rotation:    Transform::identity(),
            scaling:     Transform::identity(),
            triangles: tris,
        }
    }

    pub fn from_resource_file(filename: &str)
        -> Result<Object, Box<error::Error>>
    {
        Object::from_file(&Path::new(consts::RES_DIR_PATH).join(filename))
    }

    pub fn from_file(filename: &Path) -> Result<Object, Box<error::Error>> {
        let f = try!(fs::File::open(filename));
        let reader = io::BufReader::new(f);
        let mut lines = reader.lines();

        let num_tris = {
            let line = lines.next()
                .unwrap_or(Err(io::Error::new(
                    io::ErrorKind::Other,
                    "no triangle count found"
                )));
            line?.parse::<usize>()?
        };

        let mut pts = Vec::with_capacity(3);
        let mut tris = Vec::with_capacity(num_tris);
        for line_res in lines {
            let line = line_res?;
            let line = line.split("#").next().unwrap();
            if line.len() == 0 { continue }

            pts.push(Point::from_vec(
                line.split(' ')
                    .filter(|s| s.len() > 0)
                    .map(|s| s.parse::<Coord>())
                    .collect::<Result<Vec<_>, _>>()?
            ));

            if pts.len() == 3 {
                tris.push(trigon![pts[0], pts[1], pts[2]]);
                pts.clear();
            }
        }

        if tris.len() == num_tris {
            Ok(Object::new(tris))
        } else {
            Err(From::from(format!(
                "expected {} tris, found {}",
                num_tris,
                tris.len()
            )))
        }
    }

    pub fn render<S: Screen>(&self, renderer: &mut Renderer<S>) {
        let world_transform = self.world_transform();
        for t in &self.triangles {
            renderer.fill_triangle(*t * world_transform);
        }
    }

    fn world_transform(&self) -> Transform {
        self.translation * self.rotation * self.scaling
    }


    pub fn translate(&mut self, off: Point) {
        self.translation = Transform::translate(off) * self.translation;
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        self.scaling = Transform::scale(x, y, z) * self.scaling;
    }

    pub fn rotate_x(&mut self, theta: f64) {
        self.rotation = Transform::rotate_x(theta) * self.rotation;
    }

    pub fn rotate_y(&mut self, theta: f64) {
        self.rotation = Transform::rotate_y(theta) * self.rotation;
    }

    pub fn rotate_z(&mut self, theta: f64) {
        self.rotation = Transform::rotate_z(theta) * self.rotation;
    }


    pub fn translated(mut self, off: Point) -> Object {
        self.translation = Transform::translate(off) * self.translation;
        self
    }

    pub fn scaled(mut self, x: f64, y: f64, z: f64) -> Object {
        self.scaling = Transform::scale(x, y, z) * self.scaling;
        self
    }

    pub fn rotated_x(mut self, theta: f64) -> Object {
        self.rotation = Transform::rotate_x(theta) * self.rotation;
        self
    }

    pub fn rotated_y(mut self, theta: f64) -> Object {
        self.rotation = Transform::rotate_y(theta) * self.rotation;
        self
    }

    pub fn rotated_z(mut self, theta: f64) -> Object {
        self.rotation = Transform::rotate_z(theta) * self.rotation;
        self
    }
}
