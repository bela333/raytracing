use crate::ray_resolver::RayResolver;
use crate::utilities::{SceneData, Vector3};

pub trait Renderer<T: RayResolver>{
    fn render(&self, start: Vector3, end: Vector3, scene: SceneData) -> Vector3;
}