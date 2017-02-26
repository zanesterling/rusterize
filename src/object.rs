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
}
