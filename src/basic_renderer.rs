use crate::renderer::Renderer;
use crate::utilities::{Vector3, SceneData};
use crate::ray_resolver::RayResolver;

pub struct BasicRenderer<T>{
    pub resolver: T
}

impl<T: RayResolver> Renderer<T> for BasicRenderer<T>{
    fn render(&self, start: Vector3, end: Vector3, scene: SceneData) -> Vector3 {
        let result = self.resolver.resolve(start, end, scene);
        match result{
            None => Vector3::zero(),
            Some(v) => {
                v.color
                .multiply(Vector3::new(1f32, 1f32, 1f32).subtract(v.pos).normalized().dot(v.normal))
            }
        }
    }
}