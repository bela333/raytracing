use crate::{
    ray_resolvers::ray_resolver::{MaterialType, RayResolver, RayResult},
    utilities::{SceneData, Vector3},
};

pub struct Dummy {}

impl RayResolver for Dummy {
    fn resolve(
        &self,
        pos: Vector3,
        dir: Vector3,
        _refraction: bool,
        _scene: SceneData,
    ) -> Option<RayResult> {
        Some(RayResult::new(
            pos,
            Vector3::from_single(1.0),
            Vector3::zero().subtract(dir),
            Vector3::zero(),
            MaterialType::Diffuse,
        ))
    }
}
