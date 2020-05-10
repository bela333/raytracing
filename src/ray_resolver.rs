use crate::utilities::{Vector3, SceneData};

pub trait RayResolver{
    fn resolve(&self, pos: Vector3, dir: Vector3, scene: SceneData) -> Option<RayResult>;
}

pub enum MaterialType{
    Diffuse,
    Reflective
}

pub struct RayResult{
    pub pos: Vector3,
    pub color: Vector3,
    pub normal: Vector3,
    pub emit: Vector3,
    pub t: MaterialType
}

impl RayResult{
    pub fn new(pos: Vector3, color: Vector3, normal: Vector3, emit: Vector3, t: MaterialType) -> Self{
        Self{pos, color, normal, emit, t}
    }

    pub fn empty() -> Self{
        Self::new(Vector3::zero(), Vector3::zero(), Vector3::zero(), Vector3::zero(), MaterialType::Diffuse)
    }
}