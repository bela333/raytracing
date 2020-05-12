use crate::ray_resolver::{RayResolver, RayResult, MaterialType};
use crate::utilities::{Vector3, SceneData};


pub struct RayMarcher{
    pub max_steps: u32,
    pub max_distance: f32,
    pub epsilon: f32
}


pub struct SDFResult{
    dist: f32,
    color: Vector3,
    emit: Vector3,
    t: MaterialType
}

impl SDFResult{
    pub fn new(dist: f32, color: Vector3, emit: Vector3, t: MaterialType) -> Self{
        Self{dist, color, emit, t}
    }

    pub fn union(self, a: Self) -> Self{
        if self.dist < a.dist {
            self
        }else{
            a
        }
    }


    pub fn sphere_dist(p: Vector3, pos: Vector3, radius: f32) -> f32{
        p.subtract(pos).length()-radius
    }

    pub fn plane_dist(p: Vector3, height: f32, thickness: f32) -> f32{
        (p.y-height).abs()-thickness
    }

    pub fn box_dist(p: Vector3, b: f32) -> f32{
        let q = p.abs().subtract(Vector3::from_single(b));
        q.max(Vector3::from_single(0f32)).length() + q.y.max(q.z).max(q.x).min(0f32)
    }
}

impl RayMarcher{
    

    fn get_sdf(&self, p: Vector3) -> SDFResult{
        let _sphere1 = SDFResult::sphere_dist(p, Vector3::new(-1f32, 0f32, 3f32), 1f32);
        let _sphere2 = SDFResult::sphere_dist(p, Vector3::new(1f32, 0f32, 3f32), 1f32);
        let _sphere3 = SDFResult::sphere_dist(p, Vector3::new(0f32, 0f32, -1.2f32), 1f32);
        let _plane = SDFResult::plane_dist(p, -1f32, self.epsilon*2f32);
        let _lamp = SDFResult::plane_dist(p, 3f32, self.epsilon*2f32);
        let _cube = -SDFResult::box_dist(p, 5f32);
        
        //Red sphere
        let sphere1 = SDFResult::new(_sphere1, Vector3::from_int(0xd65c33), Vector3::zero(), MaterialType::Diffuse);
        //Reflective sphere
        let sphere2 = SDFResult::new(_sphere2, Vector3::new(1f32, 1f32, 1f32), Vector3::zero(), MaterialType::Reflective);
        //Blue sphere
        let sphere3 = SDFResult::new(_sphere3, Vector3::from_int(0x6d59d4), Vector3::zero(), MaterialType::Diffuse);
        let plane = SDFResult::new(_plane, Vector3::from_int(0x86de5d), Vector3::zero(), MaterialType::Diffuse);
        let lamp = SDFResult::new(
            _lamp,
            Vector3::new(0f32, 0f32, 0f32),
            Vector3::from_int(0xc7fff6),
            MaterialType::Diffuse
        );
        let cube = SDFResult::new(_cube, Vector3::from_single(1f32), Vector3::zero(), MaterialType::Diffuse);

        

        sphere1.union(plane).union(lamp).union(sphere2).union(cube).union(sphere3)
    }

    pub fn get_normal(&self, pos: Vector3) -> Vector3 {
        let x_probe = Vector3::new(self.epsilon, 0f32, 0f32);
        let x_delta = self.get_sdf(pos.add(x_probe)).dist - self.get_sdf(pos.subtract(x_probe)).dist;

        let y_probe = Vector3::new(0f32, self.epsilon, 0f32);
        let y_delta = self.get_sdf(pos.add(y_probe)).dist - self.get_sdf(pos.subtract(y_probe)).dist;

        let z_probe = Vector3::new(0f32, 0f32, self.epsilon);
        let z_delta = self.get_sdf(pos.add(z_probe)).dist - self.get_sdf(pos.subtract(z_probe)).dist;

        Vector3::new(x_delta, y_delta, z_delta).normalized()
    }
}

impl RayResolver for RayMarcher{
    fn resolve(&self, pos: Vector3, dir: Vector3, scene: SceneData) -> Option<RayResult> {
        let mut dist = 0f32;
        let mut p = pos;
        for _ in 0..self.max_steps{
            if dist > self.max_distance {
                return None;
            }
            let sdf_value = self.get_sdf(p);
            dist += sdf_value.dist;
            p = pos.add(dir.multiply(dist));
            if sdf_value.dist < self.epsilon {
                return Some(RayResult{
                    pos: p,
                    color: sdf_value.color,
                    normal: self.get_normal(p),
                    emit: sdf_value.emit,
                    t: sdf_value.t
                });
            }
        }
        None
    }
}