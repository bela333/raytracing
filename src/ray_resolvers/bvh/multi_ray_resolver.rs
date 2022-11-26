use crate::{
    ray_resolvers::ray_resolver::{RayResolver, RayResult},
    utilities::{SceneData, Vector3},
};

pub struct MultiRayResolver {
    pub inner: Vec<Box<dyn RayResolver + Sync>>,
}

impl RayResolver for MultiRayResolver {
    fn resolve(
        &self,
        pos: Vector3,
        dir: Vector3,
        refraction: bool,
        scene: SceneData,
    ) -> Vec<RayResult> {
        if self.inner.len() == 0 {
            return vec![];
        }
        if self.inner.len() == 1 {
            let ray = &self.inner[0];
            return ray.resolve(pos, dir, refraction, scene.clone());
        }
        let mut all = Vec::new();
        for ray in &self.inner {
            let result = ray.resolve(pos, dir, refraction, scene.clone());
            all.extend(result.into_iter());
        }
        return all;
    }
}
