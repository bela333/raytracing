use crate::ray_resolvers::bvh::aabb::AABBRayResolver;
use crate::{
    ray_resolvers::bvh::generate_bvh_from_file,
};

pub fn get_resolver() -> AABBRayResolver {
    println!("Building BVH");
    let r = generate_bvh_from_file("monke.obj").unwrap();
    println!("BVH done!");
    r
}
