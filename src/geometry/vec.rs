#[derive(Debug)]
#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn from (x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn plus(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::from(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z)
    }

    pub fn scale(&self, s: f64) -> Vec3 {
        Vec3::from(self.x * s, self.y * s, self.z * s)
    }

    pub fn minus(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::plus(&v1, &v2.scale(-1.0))
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
    }

    pub fn norm(&self) -> f64 {
        Vec3::dot(&self, &self).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        Vec3::scale(&self, 1.0 / self.norm())
    }
}
