use crate::ray_resolver::{MaterialType, RayResolver, RayResult};
use crate::utilities::{SceneData, Vector3};

pub struct RayMarcher {
    pub max_steps: u32,
    pub max_distance: f32,
    pub epsilon: f32,
    pub scene: fn(Vector3, bool) -> SDFResult,
}

#[derive(Clone)]
pub struct SDFResult {
    dist: f32,
    color: Vector3,
    emit: Vector3,
    t: MaterialType,
}

impl SDFResult {
    pub fn new(dist: f32, color: Vector3, emit: Vector3, t: MaterialType) -> Self {
        Self {
            dist,
            color,
            emit,
            t,
        }
    }

    pub fn union(self, a: Self) -> Self {
        if self.dist < a.dist {
            self
        } else {
            a
        }
    }

    pub fn sphere_dist(p: Vector3, pos: Vector3, radius: f32) -> f32 {
        p.subtract(pos).length() - radius
    }

    pub fn plane_dist(p: Vector3, height: f32, thickness: f32) -> f32 {
        (p.y - height).abs() - thickness
    }

    pub fn box_dist(p: Vector3, b: f32) -> f32 {
        let q = p.abs().subtract(Vector3::from_single(b));
        q.max(Vector3::from_single(0f32)).length() + q.y.max(q.z).max(q.x).min(0f32)
    }
}

impl RayMarcher {
    fn get_sdf(&self, p: Vector3, refraction: bool) -> SDFResult {
        let scene = self.scene;
        let mut v = scene(p, refraction);
        if refraction {
            v.dist = -v.dist;
        }
        return v;
    }

    pub fn get_normal(&self, pos: Vector3, refraction: bool) -> Vector3 {
        let x_probe = Vector3::new(self.epsilon, 0f32, 0f32);
        let x_delta = self.get_sdf(pos.add(x_probe), refraction).dist
            - self.get_sdf(pos.subtract(x_probe), refraction).dist;

        let y_probe = Vector3::new(0f32, self.epsilon, 0f32);
        let y_delta = self.get_sdf(pos.add(y_probe), refraction).dist
            - self.get_sdf(pos.subtract(y_probe), refraction).dist;

        let z_probe = Vector3::new(0f32, 0f32, self.epsilon);
        let z_delta = self.get_sdf(pos.add(z_probe), refraction).dist
            - self.get_sdf(pos.subtract(z_probe), refraction).dist;

        Vector3::new(x_delta, y_delta, z_delta).normalized()
    }
}

impl RayResolver for RayMarcher {
    fn resolve(
        &self,
        pos: Vector3,
        dir: Vector3,
        refraction: bool,
        _: SceneData,
    ) -> Option<RayResult> {
        let mut dist = 0f32;
        let mut p = pos;
        for _ in 0..self.max_steps {
            if dist > self.max_distance {
                return None;
            }
            let sdf_value = self.get_sdf(p, refraction);
            dist += sdf_value.dist;
            p = pos.add(dir.multiply(dist));
            if sdf_value.dist < self.epsilon {
                return Some(RayResult {
                    pos: p,
                    color: sdf_value.color,
                    normal: self.get_normal(p, refraction),
                    emit: sdf_value.emit,
                    t: sdf_value.t,
                });
            }
        }
        None
    }
}
