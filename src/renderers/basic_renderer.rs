use crate::ray_resolvers::ray_resolver::RayResolver;
use crate::renderers::renderer::Renderer;
use crate::utilities::{SceneData, Vector3};

pub struct BasicRenderer<T> {
    pub resolver: T,
}

impl<T: RayResolver> Renderer<T> for BasicRenderer<T> {
    fn render(&self, start: Vector3, dir: Vector3, scene: SceneData, _: u32, _: u32) -> Vector3 {
        let result = self.resolver.resolve(start, dir, false, scene);
        match result.into_iter().min_by(|res1, res2|start.subtract(res1.pos).length_squared().total_cmp(&start.subtract(res2.pos).length_squared())) {
            None => Vector3::zero(),
            Some(v) => {
                let base_color = Vector3::new(1.0, 1.0, 1.0);
                let lamp = Vector3::new(5.0, 5.0, 0.0);
                let ambient = base_color.multiply(0.25);
                let lamp_dir = lamp.subtract(v.pos);
                let diffuse = base_color.multiply(
                    lamp_dir
                        .normalized()
                        .dot(v.normal),
                );
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
