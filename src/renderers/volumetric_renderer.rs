use std::convert::TryInto;
use std::f32::consts::E;

use crate::ray_resolvers::ray_resolver::{RayResolver, RayResult};
use crate::renderers::renderer::Renderer;
use crate::utilities::{SceneData, Vector3};

pub struct VolumetricRenderer<T> {
    pub resolver: T,
    pub lamp: Vector3,
    pub divisions: usize,
    pub density: f32
}

impl<T: RayResolver> Renderer<T> for VolumetricRenderer<T> {
    fn render(&self, start: Vector3, dir: Vector3, scene: SceneData, _: u32, _: u32) -> Vector3 {
        let result = self.resolver.resolve(start, dir, false, scene.clone());
        if result.len() > 0 {

            let first_hit = result.iter().min_by(|hit1, hit2|start.subtract(hit1.pos).length_squared().total_cmp(&start.subtract(hit2.pos).length_squared())).unwrap();
            let last_hit = result.iter().max_by(|hit1, hit2|start.subtract(hit1.pos).length_squared().total_cmp(&start.subtract(hit2.pos).length_squared())).unwrap();

            let step_dist = last_hit.pos.subtract(first_hit.pos).length()/self.divisions as f32;
            let mut total_power = 0.0;
            let mut total_dist = 0.0;
            for i in 0..self.divisions {
                let t = i as f32/self.divisions as f32;
                let p = first_hit.pos.multiply(1.0-t).add(last_hit.pos.multiply(t));
                //Is the current point inside the mesh?
                let hits = self.resolver.resolve(p, Vector3::new(0.0, 1.0, 0.0), false, scene.clone());
                if hits.len()%2 == 1 {
                    //Inside mesh
                    let hits = self.resolver.resolve(p, self.lamp.subtract(p).normalized(), false, scene.clone());
                    total_dist += step_dist;
                    if let Some(last_hit) = hits.iter().max_by(|hit1, hit2|p.subtract(hit1.pos).length_squared().total_cmp(&p.subtract(hit2.pos).length_squared())){
                        //TODO: Measure distance to lamp correctly
                        let dist_to_lamp = last_hit.pos.subtract(p).length();
                        let pow = E.powf((-total_dist-dist_to_lamp)*self.density); //Power remaining after entering the mesh
                        total_power += pow;
                    }
                    

                }else{
                    //Outside mesh

                }
            }

            


            Vector3::from_single(total_power/self.divisions as f32)
        }else{
            Vector3::from_single(1.0)
        }
    }
    fn needs_toneing() -> bool {
        true
    }
}
