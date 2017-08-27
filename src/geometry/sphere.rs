use super::vec::Vec3;
use super::ray::Ray;

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
