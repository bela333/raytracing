use crate::ray_resolver::{MaterialType, RayResolver};
use crate::renderer::Renderer;
use crate::utilities::{SceneData, Vector3};
use rand_distr::{Distribution, Uniform};

pub struct PathTracer<T> {
    pub resolver: T,
    pub bounces: u32,
    pub samples: u32,
    pub epsilon: f32,
    pub contrast: f32,
    pub brightness: f32,
    pub depth_of_field: f32,
    pub dof_distr: Uniform<f32>,
    pub dof: bool,
}

fn find_outgoing(incoming: Vector3, normal: Vector3, material: MaterialType) -> Vector3 {
    match material {
        MaterialType::Diffuse => Vector3::random_on_hemisphere(normal),
        MaterialType::Reflective => incoming.reflect(normal),
        MaterialType::Lens => Vector3::zero().subtract(normal),
        MaterialType::Glass(ior) => incoming.refract(normal, ior),
    }
}

impl<T: RayResolver> PathTracer<T> {
    fn render_sample(&self, start: &Vector3, dir: &Vector3, scene: &SceneData) -> (Vector3, u32) {
        let mut start = *start;
        let mut dir = *dir;

        if self.dof {
            let p = dir.multiply(self.depth_of_field).add(start);
            let mut rng = rand::thread_rng();
            start = start.add(Vector3::new(
                self.dof_distr.sample(&mut rng),
                self.dof_distr.sample(&mut rng),
                self.dof_distr.sample(&mut rng),
            ));
            dir = p.subtract(start).normalized();
        }

        let mut emit = Vector3::zero();
        let mut rad = Vector3::new(1f32, 1f32, 1f32);
        let mut rng = rand::thread_rng();
        let mut refraction = false;
        for i in 0..self.bounces {
            let random = Uniform::new(0f32, 1f32).sample(&mut rng);
            let dust_dist = -random.ln() * scene.fog_amount;
            match { self.resolver.resolve(start, dir, refraction, scene.clone()) } {
                None => {
                    //return (emit, i)
                    if scene.fog {
                        start = start.add(dir.multiply(dust_dist));
                        dir = Vector3::random_on_sphere();
                    } else {
                        return (emit, i);
                    }
                }
                Some(ray) => {
                    if scene.fog && ray.pos.subtract(start).length() > dust_dist {
                        start = start.add(dir.multiply(dust_dist));
                        dir = Vector3::random_on_sphere();
                        continue;
                    }
                    emit = emit.add(rad.comp_multiply(ray.emit));
                    dir = find_outgoing(ray.pos.subtract(start).normalized(), ray.normal, ray.t);
                    rad = rad.comp_multiply(ray.color.multiply(ray.normal.dot(dir)));
                    start = ray.pos.add(ray.normal.multiply(self.epsilon * 2f32));
                    if rad.x == 0f32 && rad.y == 0f32 && rad.z == 0f32 {
                        break;
                    }
                    if dir.dot(ray.normal.clone()) < 0f32 {
                        //Refraction
                        refraction = !refraction;
                        start = start.subtract(ray.normal.multiply(4f32 * self.epsilon))
                    }
                }
            }
        }
        (emit, self.bounces - 1)
    }
}

impl<T: RayResolver> Renderer<T> for PathTracer<T> {
    fn render(
        &self,
        start: Vector3,
        dir: Vector3,
        scene: SceneData,
        width: u32,
        height: u32,
    ) -> Vector3 {
        let mut o = Vector3::zero();
        let mut rng = rand::thread_rng();
        let distr = Uniform::new(-0.5 / height as f32, 0.5 / height as f32);
        for _ in 0..self.samples {
            let aa_jitter = Vector3::new(distr.sample(&mut rng), distr.sample(&mut rng), 0f32);
            let dir_aa = dir.clone().add(aa_jitter);
            let (c, _) = self.render_sample(&start, &dir_aa, &scene);
            o = o.add(c);
        }
        o.multiply(1f32 / (self.samples as f32))
    }
    fn needs_toneing() -> bool {
        true
    }
}
