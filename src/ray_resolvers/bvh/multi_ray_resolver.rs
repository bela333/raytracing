use std::cmp::Ordering;

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
    ) -> Option<RayResult> {
        if self.inner.len() == 0 {
            return None
        }
        if self.inner.len() == 1 {
            let ray = &self.inner[0];
            return ray.resolve(pos, dir, refraction, scene.clone());
        }
        let mut closest = None;
        let mut closest_distance = 0.0;
        for ray in &self.inner{
            let result = ray.resolve(pos, dir, refraction, scene.clone());
            match result {
                Some(result) => {
                    let distance = result.pos.subtract(pos).dot(dir);
                    if closest.is_none() || distance < closest_distance {
                        closest = Some(result);
                        closest_distance = distance;
                    }
                },
                None => (),
            }
        }
        return closest;
    }
}
