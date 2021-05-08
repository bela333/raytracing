use std::{cmp::Ordering, f32::EPSILON, fs::File, io::{BufRead, BufReader}, path::Path, usize};

use obj::{Obj, load_obj};

use crate::{error::Error, ray_resolver::MaterialType, utilities::{Components, Vector3}};

use self::{aabb::AABBRayResolver, multi_ray_resolver::MultiRayResolver, triangle::{Triangle, TriangleResolver}};

pub mod aabb;
pub mod dummy;
pub mod multi_ray_resolver;
pub mod triangle;

pub fn generate_bvh_from_file<P: AsRef<Path>>(filename: P) -> Result<AABBRayResolver, Error>{
    let file = BufReader::new(File::open(filename)?);
    generate_bvh_from_bufread(file)
}
pub fn generate_bvh_from_bufread<T: BufRead>(buffer: T) -> Result<AABBRayResolver, Error>{
    let triangles = triangles_from_bufread(buffer)?;
    generate_bvh(triangles)
}

pub fn triangles_from_file<P: AsRef<Path>>(filename: P) -> Result<Vec<Triangle>, Error>{
    let file = BufReader::new(File::open(filename)?);
    triangles_from_bufread(file)
}

pub fn triangles_from_bufread<T: BufRead>(buffer: T) -> Result<Vec<Triangle>, Error>{
    let obj: Obj = load_obj(buffer)?;
    let triangles: Vec<Triangle> = obj.indices
        .chunks(3)
        .map(|i| {
            match i{
                [i0, i1, i2] => (obj.vertices[*i0 as usize], obj.vertices[*i1 as usize], obj.vertices[*i2 as usize]),
                _ => panic!("Couldn't load mesh")
            }
            
        })
        .map(|(v0, v1, v2)| {
            let n0 = Vector3::new(-v0.normal[0], v0.normal[1], v0.normal[2]);
            let n1 = Vector3::new(-v1.normal[0], v1.normal[1], v1.normal[2]);
            let n2 = Vector3::new(-v2.normal[0], v2.normal[1], v2.normal[2]);
            let v0 = Vector3::new(-v0.position[0], v0.position[1], v0.position[2]);
            let v1 = Vector3::new(-v1.position[0], v1.position[1], v1.position[2]);
            let v2 = Vector3::new(-v2.position[0], v2.position[1], v2.position[2]);
            Triangle::new_with_normal(v2, v1, v0,n0, n1, n2, Vector3::from_single(1.0), Vector3::zero(), MaterialType::Diffuse)
        }).collect();
    Ok(triangles)
}

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
        let mut aabb = triangle.bounds();
        let margin: Vector3 = Vector3::from_single(0.0001); //Add a small margin around the triangle
        aabb.min = aabb.min.subtract(margin);
        aabb.max = aabb.max.add(margin);
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