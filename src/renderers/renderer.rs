use crate::{
    ray_resolvers::ray_resolver::RayResolver,
    utilities::{SceneData, Vector3},
};

pub trait Renderer<T: RayResolver> {
    fn render(
        &self,
        start: Vector3,
        end: Vector3,
        scene: SceneData,
        width: u32,
        height: u32,
    ) -> Vector3;
    fn needs_toneing() -> bool;
}
