use crate::renderer::Renderer;
use crate::utilities::{Vector3, SceneData};
use crate::ray_resolver::{RayResult, RayResolver, MaterialType};

pub struct PathTracer<T>{
    pub resolver: T,
    pub bounces: u32,
    pub samples: u32,
    pub epsilon: f32,
    pub contrast: f32,
    pub brightness: f32
}

fn find_outgoing(incoming: Vector3, normal: Vector3, material: MaterialType) -> Vector3{
    match material {
        MaterialType::Diffuse => Vector3::random_on_hemisphere(normal),
        MaterialType::Reflective => incoming.reflect(normal)
    }
}

impl<T: RayResolver> PathTracer<T>{
    fn render_sample(&self, start: &Vector3, dir: &Vector3, scene: &SceneData) -> Vector3{
        let mut start = *start;
        let mut dir = *dir;

        let mut emit = Vector3::zero();
        let mut rad = Vector3::new(1f32, 1f32, 1f32);
        for _ in 0..self.bounces{
            match self.resolver.resolve(start, dir, scene.clone()) {
                None => break,
                Some(ray) => {
                    emit = emit.add(rad.comp_multiply(ray.emit));
                    rad = rad.comp_multiply(ray.color.multiply(ray.normal.dot(dir.multiply(-1f32))));
                    dir = find_outgoing(ray.pos.subtract(start).normalized(), ray.normal, ray.t);
                    start = ray.pos.add(ray.normal.multiply(self.epsilon*4f32));
                }
            }
        }
        emit
    }
}

impl<T: RayResolver> Renderer<T> for PathTracer<T>{
    fn render(&self, start: Vector3, dir: Vector3, scene: SceneData) -> Vector3 {
        let mut o = Vector3::zero();
        for _ in 0..self.samples{
            let c = self.render_sample(&start, &dir, &scene);
            o = o.add(c);
        }
        o.multiply(1f32/(self.samples as f32)).pow(self.contrast).add_scalar(self.brightness)
    }
}