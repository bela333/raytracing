use crate::{bvh::{aabb::{AABB, AABBRayResolver}, dummy::Dummy, multi_ray_resolver::MultiRayResolver}, ray_marcher::{self, SDFResult}, ray_resolver::{MaterialType}, utilities::Vector3};

fn raymarcher_scene(p: Vector3, refraction: bool) -> SDFResult {

    let sphere1 = SDFResult::new(
        SDFResult::sphere_dist(p, Vector3::new(0f32, 0.0f32, 4f32), 1.5),
       Vector3::from_single(1.0),
       Vector3::zero(),
       MaterialType::Diffuse);
    let sphere2 = SDFResult::new(
        SDFResult::sphere_dist(p, Vector3::new(-3f32, 0.0f32, 4f32), 1.5),
       Vector3::from_single(1.0),
       Vector3::zero(),
       MaterialType::Reflective);
    let sphere3 = SDFResult::new(
        SDFResult::sphere_dist(p, Vector3::new(3f32, 0.0f32, 4f32), 1.5),
        Vector3::from_int(0xebbdb9).srgb(),
        Vector3::zero(),
        MaterialType::Glass(if refraction{1.52/1.000293}else{1.000293/1.52}));
    let sphere4 = SDFResult::new(
        SDFResult::sphere_dist(p, Vector3::new(3f32, 0.0f32, 4f32), 1.5),
       Vector3::from_single(1.0),
       Vector3::zero(),
       MaterialType::Glass(if refraction{1.52/1.000293}else{1.000293/1.52}));

    sphere1.union(sphere2).union(sphere3)
}

pub fn get_resolver() -> MultiRayResolver{
    let resolver1 = {
        let dummy = Dummy{};
        let c = Vector3::new(0.0, 0.0, 4.0);
        let r = 1.5;
        let r = Vector3::from_single(r);
        let aabb = AABB{
            min: c.subtract(r),
            max: c.add(r)
        };
        AABBRayResolver::new(aabb, dummy)
    };
    let resolver2 = {
        let dummy = Dummy{};
        let c = Vector3::new(-3.0, -3.0, 4.0);
        let r = 1.5;
        let r = Vector3::from_single(r);
        let aabb = AABB{
            min: c.subtract(r),
            max: c.add(r)
        };
        AABBRayResolver::new(aabb, dummy)
    };
    let resolver3 = {
        let dummy = Dummy{};
        let c = Vector3::new(-3.0, -3.0, 6.0);
        let r = 1.5;
        let r = Vector3::from_single(r);
        let aabb = AABB{
            min: c.subtract(r),
            max: c.add(r)
        };
        AABBRayResolver::new(aabb, dummy)
    };
    MultiRayResolver{
        inner: vec![
            Box::new(resolver1),
            Box::new(resolver2),
            Box::new(resolver3),
        ]
    }
}