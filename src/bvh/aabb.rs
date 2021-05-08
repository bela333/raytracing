use crate::{ray_resolver::RayResolver, utilities::Vector3};

#[derive(Clone, Copy)]
pub struct AABB{
    pub min: Vector3,
    pub max: Vector3
}

impl AABB{
    fn trace(&self, pos: &Vector3, dir: &Vector3) -> Option<Vector3>{
        let invdir = dir.reciprocal();
        let (mut tmin, mut tmax) = if invdir.x >= 0.0 {
            let tmin = (self.min.x - pos.x) * invdir.x;
            let tmax = (self.max.x - pos.x) * invdir.x;
            (tmin, tmax)
        }else{
            let tmin = (self.max.x - pos.x) * invdir.x;
            let tmax = (self.min.x - pos.x) * invdir.x;
            (tmin, tmax)
        };
        let (tymin, tymax) = if invdir.y >= 0.0 {
            let tymin = (self.min.y - pos.y) * invdir.y;
            let tymax = (self.max.y - pos.y) * invdir.y;
            (tymin, tymax)
        }else{
            let tymin = (self.max.y - pos.y) * invdir.y;
            let tymax = (self.min.y - pos.y) * invdir.y;
            (tymin, tymax)
        };

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let (tzmin, tzmax) = if invdir.z >= 0.0 {
            let tzmin = (self.min.z - pos.z) * invdir.z;
            let tzmax = (self.max.z - pos.z) * invdir.z;
            (tzmin, tzmax)
        }else{
            let tzmin = (self.max.z - pos.z) * invdir.z;
            let tzmax = (self.min.z - pos.z) * invdir.z;
            (tzmin, tzmax)
        };

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        let t = if tmin < 0.0 {
            tmax
        }else{
            tmin
        };
        if tmax < 0.0 {
            return None;
        }
        Some(dir.multiply(t).add(pos.clone()))
        

    }
}

pub struct AABBRayResolver<T>{
    pub aabb: AABB,
    pub inner: Box<T>
}

impl<T: RayResolver> AABBRayResolver<T>{
    pub fn new(aabb: AABB, inner: T) -> Self{
        let inner = Box::new(inner);
        Self{
            aabb: aabb,
            inner: inner
        }
    }
}

impl<T: RayResolver> RayResolver for AABBRayResolver<T>{
    fn resolve(
        &self,
        pos: Vector3,
        dir: Vector3,
        refraction: bool,
        scene: crate::utilities::SceneData,
    ) -> Option<crate::ray_resolver::RayResult> {
        match self.aabb.trace(&pos, &dir) {
            Some(pos) => self.inner.resolve(pos, dir, refraction, scene),
            None => None
        }
    }
}