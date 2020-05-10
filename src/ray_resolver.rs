use crate::utilities::{Vector3, SceneData};

pub trait RayResolver{
    fn resolve(&self, pos: Vector3, dir: Vector3, scene: SceneData) -> Option<RayResult>;
}

pub struct RayResult{
    pub pos: Vector3,
    pub color: Vector3,
    pub normal: Vector3,
}

impl RayResult{
    pub fn new(pos: Vector3, color: Vector3, normal: Vector3) -> Self{
        Self{pos, color, normal}
    }

    pub fn empty() -> Self{
        Self::new(Vector3::zero(), Vector3::zero(), Vector3::zero())
    }
}