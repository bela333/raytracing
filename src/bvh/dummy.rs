use crate::{
    ray_resolver::{RayResolver, RayResult},
    utilities::Vector3,
};

pub struct Dummy {}

impl RayResolver for Dummy {
    fn resolve(
        &self,
        pos: crate::utilities::Vector3,
        dir: crate::utilities::Vector3,
        refraction: bool,
        scene: crate::utilities::SceneData,
    ) -> Option<crate::ray_resolver::RayResult> {
        Some(RayResult::new(
            pos,
            Vector3::from_single(1.0),
            Vector3::zero().subtract(dir),
            Vector3::zero(),
            crate::ray_resolver::MaterialType::Diffuse,
        ))
    }
}
