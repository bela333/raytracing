use std::cmp::Ordering;

use crate::{error::Error, ray_resolver::RayResolver, utilities::Components};

use self::{aabb::{AABB, AABBRayResolver}, multi_ray_resolver::MultiRayResolver, triangle::{Triangle, TriangleResolver}};

pub mod aabb;
pub mod dummy;
pub mod multi_ray_resolver;
pub mod triangle;

pub fn generate_bvh(triangles: Vec<Triangle>) -> Result<AABBRayResolver, Error>{
    _generate_bvh(triangles, Components::X)
}

fn _generate_bvh(mut triangles: Vec<Triangle>, orientation: Components) -> Result<AABBRayResolver, Error>{
    if triangles.len() < 1 {
        return Err(Error::new("BVH generation requires atleast 2 triangles".to_string()));
    }
    if triangles.len() == 1{
        //Return single triangle
        let triangle = triangles[0].clone();
        let aabb = triangle.bounds();
        let inner = TriangleResolver{
            triangle
        };
        let resolver = AABBRayResolver::new(aabb, inner);
        return Ok(resolver);
    }
    //Divide triangles among the median
    let (t1, t2) = {
        let index = triangles.len()/2;
        triangles.select_nth_unstable_by(index,
            |a, b|{
                let a_pos = a.centroid.get_component(orientation);
                let b_pos = b.centroid.get_component(orientation);
                a_pos.partial_cmp(&b_pos).unwrap_or(Ordering::Equal)
            }
        );
        let (t1, t2) = triangles.split_at(index);
        (t1.to_vec(), t2.to_vec())
    };

    //Recursive generate BVH for each branch
    let bvh1 = _generate_bvh(t1, orientation.next())?;
    let bvh2 = _generate_bvh(t2, orientation.next())?;
    //Calculate size of current AABB
    let bounds = bvh1.aabb.union(&bvh2.aabb);
    let bvh1 = Box::new(bvh1);
    let bvh2 = Box::new(bvh2);
    //Create MultiRayResolver
    let multi = MultiRayResolver{
        inner: vec![bvh1, bvh2]
    };
    //Create current AABB and return
    Ok(AABBRayResolver::new(bounds, multi))
    
}