use renderer::Renderer;
use screen::Screen;
use types::*;

pub struct Object {
    translate_transform:    Transform,
    rotate_scale_transform: Transform,
    triangles: Vec<Triangle>,
}

#[allow(dead_code)]
impl Object {
    pub fn new(tris: Vec<Triangle>) -> Object {
        Object {
            translate_transform:    Transform::identity(),
            rotate_scale_transform: Transform::identity(),
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
        self.rotate_scale_transform * self.translate_transform
    }
}
