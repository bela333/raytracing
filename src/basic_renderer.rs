use crate::renderer::Renderer;
use crate::utilities::{Vector3, SceneData};
use crate::ray_resolver::RayResolver;

pub struct BasicRenderer<T>{
    pub resolver: T
}

impl<T: RayResolver> Renderer<T> for BasicRenderer<T>{
    fn render(&self, start: Vector3, dir: Vector3, scene: SceneData, _: u32, _: u32) -> Vector3 {
        let result = self.resolver.resolve(start, dir, scene);
        match result{
            None => Vector3::zero(),
            Some(v) => {
                
                let lamp = Vector3::new(1f32, 1f32, 1f32);
                let ambient = v.color.multiply(0.1);
                let lamp_dir = lamp.subtract(v.pos);
                let diffuse = v.color.multiply(lamp_dir.normalized().dot(v.normal).max(0.0));
                let specular = lamp_dir.reflect(v.normal).dot(dir.multiply(-1f32));
                let specular = if lamp_dir.dot(v.normal) < 0f32 {0f32}else{specular};
                let specular = if specular < 0f32 {0f32}else{specular};
                let specular = specular.powf(100f32);
                let specular = Vector3::from_single(specular);
                if v.emit.x == 0.0 || v.emit.y == 0.0 || v.emit.z == 0.0{
                    ambient.add(diffuse).add(specular).add(v.emit)
                }
                else{
                    v.emit
                }
            }
        }
    }
    fn needs_toneing() -> bool {true}
}