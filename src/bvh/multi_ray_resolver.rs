use std::cmp::Ordering;

use crate::ray_resolver::RayResolver;

pub struct MultiRayResolver {
    pub inner: Vec<Box<dyn RayResolver + Sync>>,
}

impl RayResolver for MultiRayResolver {
    fn resolve(
        &self,
        pos: crate::utilities::Vector3,
        dir: crate::utilities::Vector3,
        refraction: bool,
        scene: crate::utilities::SceneData,
    ) -> Option<crate::ray_resolver::RayResult> {
        let r = self
            .inner
            .iter()
            .map(|r| r.resolve(pos, dir, refraction, scene.clone()))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .map(|r| {
                let dist = r.pos.subtract(pos).dot(dir);
                (r, dist)
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
        match r {
            Some((r, _)) => Some(r),
            None => None,
        }
    }
}
