

use crate::{bvh::{aabb::{AABB, AABBRayResolver}, dummy::Dummy, generate_bvh_from_file, multi_ray_resolver::MultiRayResolver, triangle::{Triangle, TriangleResolver}, triangles_from_file}, ray_marcher::{self, SDFResult}, ray_resolver::{MaterialType, RayResolver}, utilities::Vector3};

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

pub fn get_resolver() -> AABBRayResolver{
    println!("Building BVH");
    let r = generate_bvh_from_file("teapot.obj").unwrap();
    println!("BVH done!");
    r
}