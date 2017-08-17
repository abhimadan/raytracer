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

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub colour: Vec3,
}

impl Sphere {
    pub fn from(center: Vec3, radius: f64, colour: Vec3) -> Sphere {
        Sphere {
            center,
            radius,
            colour,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(Vec3, f64)> {
        let a = 1.0;  // ray.direction must be normalized
        let b = Vec3::dot(&ray.direction.scale(2.0),
        &Vec3::minus(&ray.origin, &self.center));
        let c = Vec3::dot(&Vec3::minus(&ray.origin, &self.center),
        &Vec3::minus(&ray.origin, &self.center))
            - self.radius*self.radius;

        let discriminant = b*b - 4.0*a*c;
        if discriminant <= 0.0 {
            None
        } else {
            let discriminant = discriminant.sqrt();
            let t1 = (-b - discriminant) / (2.0 * a);
            let t2 = (-b + discriminant) / (2.0 * a);
            let tmin: f64 = f64::min(t1, t2);
            let tmax: f64 = f64::max(t1, t2);

            if tmin < 0.0001 {
                if tmax < 0.0001 {
                    None
                } else {
                    Some((Vec3::plus(&ray.origin, &ray.direction.scale(tmax)), tmax))
                }
            } else {
                Some((Vec3::plus(&ray.origin, &ray.direction.scale(tmin)), tmin))
            }
        }
    }
}
