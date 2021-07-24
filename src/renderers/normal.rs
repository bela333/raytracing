use crate::{ray_resolvers::ray_resolver::RayResolver, utilities::{SceneData, Vector3}};

use super::renderer::Renderer;

pub struct NormalRenderer<T> where T: RayResolver{
    pub resolver: T
}

impl<T: RayResolver> Renderer<T> for NormalRenderer<T> {
    fn render(
        &self,
        start: Vector3,
        end: Vector3,
        scene: SceneData,
        width: u32,
        height: u32,
    ) -> Vector3 {
        let result = match self.resolver.resolve(start, end, false, scene){
            Some(a) => a,
            None => return Vector3::zero(),
        };
        result.normal
    }

    fn needs_toneing() -> bool {
        true
    }
}

