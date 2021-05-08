use std::cmp::Ordering;

use crate::{ray_resolver::{MaterialType, RayResolver, RayResult}, utilities::Vector3};

use super::aabb::AABB;
const EPSILON: f32 = 0.00001;
#[derive(Clone)]
pub struct Triangle{
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
    pub normal: Vector3,
    pub centroid: Vector3,
    pub color: Vector3,
    pub emit: Vector3,
    pub t: MaterialType
}

impl Triangle {
    pub fn new(v0: Vector3, v1: Vector3, v2: Vector3, color: Vector3, emit: Vector3, t: MaterialType) -> Self{
        let v0v1 = v1.subtract(v0);
        let v0v2 = v2.subtract(v0);
        let normal = v0v1.cross(v0v2).normalized();
        let centroid = v0.add(v1).add(v2).multiply(1.0/3.0);
        Self{
            v0, v1, v2,
            normal,
            centroid,
            color,
            emit,
            t
        }
    }
    pub fn trace(&self, pos: &Vector3, dir: &Vector3) -> Option<(Vector3, f32, f32)>{
        let v0v1 = self.v1.subtract(self.v0);
        let v0v2 = self.v2.subtract(self.v0);
        let pvec = dir.cross(v0v2);
        let det = v0v1.dot(pvec);
        if det < EPSILON {
            return None;
        }
        let inv_det = 1.0 / det;
        let tvec = pos.subtract(self.v0);
        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let qvec = tvec.cross(v0v1);
        let v = dir.dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = v0v2.dot(qvec)*inv_det;
        let hit = dir.multiply(t).add(*pos);
        return Some((hit, u, v));
    }

    pub fn bounds(&self) -> AABB{
        let xmin = *[self.v0.x, self.v1.x, self.v2.x].iter().min_by(|a, b|a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap();
        let ymin = *[self.v0.y, self.v1.y, self.v2.y].iter().min_by(|a, b|a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap();
        let zmin = *[self.v0.z, self.v1.z, self.v2.z].iter().min_by(|a, b|a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap();

        let xmax = *[self.v0.x, self.v1.x, self.v2.x].iter().max_by(|a, b|a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap();
        let ymax = *[self.v0.y, self.v1.y, self.v2.y].iter().max_by(|a, b|a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap();
        let zmax = *[self.v0.z, self.v1.z, self.v2.z].iter().max_by(|a, b|a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap();
        AABB{
            min: Vector3::new(xmin, ymin, zmin),
            max: Vector3::new(xmax, ymax, zmax)
        }
    }
}

pub struct TriangleResolver{
    pub triangle: Triangle
}

impl RayResolver for TriangleResolver {
    fn resolve(
        &self,
        pos: Vector3,
        dir: Vector3,
        refraction: bool,
        scene: crate::utilities::SceneData,
    ) -> Option<RayResult> {
        match self.triangle.trace(&pos, &dir) {
            Some((hit, u, v)) => {
                Some(RayResult::new(hit, self.triangle.color, self.triangle.normal, self.triangle.emit, self.triangle.t.clone()))
            },
            None => None
        }
    }
}