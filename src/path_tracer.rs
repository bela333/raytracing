use crate::renderer::Renderer;
use crate::utilities::{Vector3, SceneData};
use crate::ray_resolver::{RayResolver, MaterialType};
use rand_distr::Uniform;

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
    fn render_sample(&self, start: &Vector3, dir: &Vector3, scene: &SceneData) -> (Vector3, u32){
        let mut start = *start;
        let mut dir = *dir;

        let mut emit = Vector3::zero();
        let mut rad = Vector3::new(1f32, 1f32, 1f32);
        for i in 0..self.bounces{
            match self.resolver.resolve(start, dir, scene.clone()) {
                None => return (emit, i),
                Some(ray) => {
                    emit = emit.add(rad.comp_multiply(ray.emit));
                    rad = rad.comp_multiply(ray.color.multiply(ray.normal.dot(dir.multiply(-1f32))));
                    dir = find_outgoing(ray.pos.subtract(start).normalized(), ray.normal, ray.t);
                    start = ray.pos.add(ray.normal.multiply(self.epsilon*2f32));
                    if rad.x == 0f32 && rad.y == 0f32 && rad.z == 0f32 {
                        break;
                    }
                }
            }
        }
        (emit, self.bounces-1)
    }
}

impl<T: RayResolver> Renderer<T> for PathTracer<T>{
    fn render(&self, start: Vector3, dir: Vector3, scene: SceneData, width: u32, height: u32) -> Vector3 {
        let mut o = Vector3::zero();
        let mut rng = rand::thread_rng();
        let distr = Uniform::new(-0.5/height as f32, 0.5/height as f32);
        for _ in 0..self.samples{
            /*let aa_jitter = Vector3::new(distr.sample(&mut rng), distr.sample(&mut rng), 0f32);
            let dir_aa = dir.clone().add(aa_jitter);*/ //Anti-aliasing is unavailable because of the sky skipping code
            let (c, i) = self.render_sample(&start, &dir, &scene);
            if i==0 {
                break;
            }
            o = o.add(c);
        }
        o.multiply(1f32/(self.samples as f32))
    }
}