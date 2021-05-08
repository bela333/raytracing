use crate::ray_resolver::RayResolver;
use crate::renderer::Renderer;
use crate::utilities::{SceneData, Vector3};

pub struct BasicRenderer<T> {
    pub resolver: T,
}

impl<T: RayResolver> Renderer<T> for BasicRenderer<T> {
    fn render(&self, start: Vector3, dir: Vector3, scene: SceneData, _: u32, _: u32) -> Vector3 {
        let result = self.resolver.resolve(start, dir, false, scene);
        match result {
            None => Vector3::zero(),
            Some(v) => {
                let lamp = Vector3::new(0.0, 0.0, -5.0);
                let ambient = v.color.multiply(0.25);
                let diffuse = v.color.multiply(
                    Vector3::new(1f32, 1f32, 1f32)
                        .subtract(v.pos)
                        .normalized()
                        .dot(v.normal),
                );
                let lamp_dir = lamp.subtract(v.pos);
                let specular = lamp_dir.reflect(v.normal).dot(dir.multiply(-1f32));
                let specular = if lamp_dir.dot(v.normal) < 0f32 {
                    0f32
                } else {
                    specular
                };
                let specular = if specular < 0f32 { 0f32 } else { specular };
                let specular = specular.powf(5f32);
                let specular = Vector3::from_single(specular);
                ambient.add(diffuse).add(specular).add(v.emit)
            }
        }
    }
    fn needs_toneing() -> bool {
        false
    }
}
