use rand_distr::{UnitSphere, Distribution};

#[derive(Copy, Clone)]
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3{
    pub fn zero() -> Self{
        Self::new(0f32, 0f32, 0f32)
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self{
            x, y, z
        }
    }

    fn restrict_value(v: f32) -> f32{
        match v{
            v if v < 0f32 => 0f32,
            v  if v > 1f32 => 1f32,
            _ => v
        }
    }

    pub fn to_color_array(&self) -> [u8;3]{
        let x = Vector3::restrict_value(self.x);
        let y = Vector3::restrict_value(self.y);
        let z = Vector3::restrict_value(self.z);
        [(x*255f32) as u8,
         (y*255f32) as u8,
         (z*255f32) as u8]
    }

    pub fn subtract(&self, a: Self) -> Self{
        Self{
            x: self.x - a.x,
            y: self.y - a.y,
            z: self.z - a.z,
        }
    }

    pub fn add(&self, a: Self) -> Self{
        Self{
            x: self.x + a.x,
            y: self.y + a.y,
            z: self.z + a.z,
        }
    }

    pub fn length_squared(&self) -> f32{
        self.dot(*self)
    }

    pub fn length(&self) -> f32{
        self.length_squared().sqrt()
    }

    pub fn multiply(&self, scalar: f32) -> Self{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn normalized(&self) -> Self{
        let length = self.length();
        self.multiply(1f32/length)
    }

    pub fn dot(&self, a: Self) -> f32{
        self.x * a.x + self.y * a.y + self.z * a.z
    }

    pub fn reflect(&self, normal: Self) -> Self{
        let a = self.dot(normal);
        let p = self.multiply(1f32/a);
        normal.multiply(2f32).add(p).normalized()
    }

    pub fn random_on_sphere() -> Self{
        let mut rng = rand::thread_rng();
        let v: [f32; 3] = UnitSphere.sample(&mut rng);
        Self::new(v[0], v[1], v[2])
    }

    pub fn random_on_hemisphere(normal: Self) -> Self{
        let p = Self::random_on_sphere();
        if p.dot(normal) < 0f32 {
            p.multiply(-1f32)
        }else{
            p
        }
    }
}

#[derive(Clone)]
pub struct SceneData{

}