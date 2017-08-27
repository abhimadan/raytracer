use super::vec::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3, // should be normalized
}

impl Ray {
    pub fn from(o: Vec3, d: Vec3) -> Ray {
        Ray {
            origin: o,
            direction: d.normalize(),
        }
    }
}
