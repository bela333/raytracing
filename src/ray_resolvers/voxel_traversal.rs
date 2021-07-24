use crate::utilities::{Vector3, SceneData};


use std::collections::HashMap;

use super::ray_resolver::{MaterialType, RayResolver, RayResult};

#[derive(Clone)]
pub struct Voxel{
    pos: (i32, i32, i32),
    color: Vector3,
    emit: Vector3,
    t: MaterialType
}

impl Voxel{
    pub fn new(pos: (i32, i32, i32), color: Vector3, emit: Vector3, t: MaterialType) -> Self{
        Voxel{
            pos, color, emit, t
        }
    }
}

pub struct VoxelTraversal{
    voxels: HashMap<(i32, i32, i32), Voxel>,
    steps: u32
}

impl VoxelTraversal{
    pub fn new(steps: u32) -> Self{
        Self{
            voxels: HashMap::new(),
            steps
        }
    }

    pub fn new_voxel(&mut self, pos: (i32, i32, i32), color: Vector3, emit: Vector3, t: MaterialType){
        self.voxels.insert(pos, Voxel::new(pos, color, emit, t));
    }

    pub fn get_voxel(&self, pos: (i32, i32, i32)) -> Option<&Voxel>{
        self.voxels.get(&pos) 
    }

    fn better_floor(v: f32) -> i32{
        if v < 0f32 {
            v.ceil() as i32
        }else {
            v as i32
        }
    }

    pub fn cast_ray(&self, pos: Vector3, dir: Vector3, scene: SceneData) -> Option<(Voxel, f32)>{
        let mut x = Self::better_floor(pos.x);
        let mut y = Self::better_floor(pos.y);
        let mut z = Self::better_floor(pos.z);

        let stepX = if dir.x >= 0f32 {1} else {-1};
        let stepY = if dir.y >= 0f32 {1} else {-1};
        let stepZ = if dir.z >= 0f32 {1} else {-1};

        let nextX = (x+stepX) as f32;
        let nextY = (y+stepY) as f32;
        let nextZ = (z+stepZ) as f32;

        let mut tMaxX = if dir.x == 0f32 {std::f32::MAX} else {(nextX - pos.x) / dir.x};
        let mut tMaxY = if dir.y == 0f32 {std::f32::MAX} else {(nextY - pos.y) / dir.y};
        let mut tMaxZ = if dir.z == 0f32 {std::f32::MAX} else {(nextZ - pos.z) / dir.z};

        let tDeltaX = if dir.x == 0f32 {std::f32::MAX} else {stepX as f32/dir.x};
        let tDeltaY = if dir.y == 0f32 {std::f32::MAX} else {stepY as f32/dir.y};
        let tDeltaZ = if dir.z == 0f32 {std::f32::MAX} else {stepZ as f32/dir.z};

        let mut t = 0f32;

        if let Some(voxel) = self.get_voxel((x, y, z)){
            return Some((voxel.clone(), t));
        }
        let mut neg_ray = false;
        let mut diffX = 0i32;
        let mut diffY = 0i32;
        let mut diffZ = 0i32;
        if dir.x < 0f32 {neg_ray = true;diffX -= 1;};
        if dir.y < 0f32 {neg_ray = true;diffY -= 1;};
        if dir.z < 0f32 {neg_ray = true;diffZ -= 1;};
        if neg_ray {
            x += diffX;
            y += diffY;
            z += diffZ;
            if let Some(voxel) = self.get_voxel((x, y, z)){
                return Some((voxel.clone(), t));
            }
        }

        for _ in 0..self.steps{
            if tMaxX < tMaxY {
                if tMaxX < tMaxZ {
                    //tMaxX is the smallest
                    x += stepX;
                    t = tMaxX;
                    tMaxX += tDeltaX;
                }else{
                    //tMaxZ is the smallest
                    z += stepZ;
                    t = tMaxZ;
                    tMaxZ += tDeltaZ;
                }
            }else{
                if tMaxY < tMaxZ {
                    //tMaxY is the smallest
                    y += stepY;
                    t = tMaxY;
                    tMaxY += tDeltaY;
                }else{
                    //tMaxZ is the smallest
                    z += stepZ;
                    t = tMaxZ;
                    tMaxZ += tDeltaZ;
                }
            }
            if let Some(voxel) = self.get_voxel((x, y, z)){
                return Some((voxel.clone(), t));
            }
        }
        None
    }
}

impl RayResolver for VoxelTraversal{
    fn resolve(&self, pos: Vector3, dir: Vector3, scene: SceneData) -> Option<RayResult> {
        let (voxel, t) = self.cast_ray(pos, dir, scene)?;
        let hit = dir.multiply(t).add(pos);
        let voxel_middle = Vector3::new(voxel.pos.0 as f32, voxel.pos.1 as f32, voxel.pos.2 as f32).add(Vector3::new(0.5, 0.5, 0.5));
        let normal = hit.subtract(voxel_middle).only_largest_component().normalized();
        Some(RayResult{
            pos: hit,
            color: voxel.color,
            normal: normal,
            emit: voxel.emit,
            t: voxel.t,
        })
    }
    
}