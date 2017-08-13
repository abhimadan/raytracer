use std::io::prelude::*;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
#[derive(Clone)]
struct Vec3 {
  x: f64,
  y: f64,
  z: f64,
}

impl Vec3 {
  fn from (x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 {
      x,
      y,
      z,
    }
  }

  fn plus(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::from(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z)
  }

  fn scale(&self, s: f64) -> Vec3 {
    Vec3::from(self.x * s, self.y * s, self.z * s)
  }

  fn minus(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::plus(&v1, &v2.scale(-1.0))
  }

  fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
  }

  fn norm(&self) -> f64 {
    Vec3::dot(&self, &self).sqrt()
  }

  fn normalize(&self) -> Vec3 {
    Vec3::scale(&self, 1.0 / self.norm())
  }
}

#[derive(Debug)]
struct Ray {
  origin: Vec3,
  direction: Vec3, // should be normalized
}

impl Ray {
  fn from(o: Vec3, d: Vec3) -> Ray {
    Ray {
      origin: o,
      direction: d.normalize(),
    }
  }
}

struct Sphere {
  center: Vec3,
  radius: f64,
  colour: Vec3,
}

impl Sphere {
  fn from(center: Vec3, radius: f64, colour: Vec3) -> Sphere {
    Sphere {
      center,
      radius,
      colour,
    }
  }

  fn intersect(&self, ray: &Ray) -> Option<(Vec3, f64)> {
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

fn main() {
  // Objects in scene
  let s1 = Sphere::from(Vec3::from(0.0, 10.0, 0.0), 5.0, Vec3::from(1.0, 1.0, 0.7));
  let s2 = Sphere::from(Vec3::from(-13.0, 20.0, 0.0), 15.0, Vec3::from(1.0, 0.3, 0.4));

  // Light sources
  let light = Vec3::from(-20.0, 10.0, 32.0);

  // View plane at (-20, 8, 20) to (20, 8, -20)
  let camera = Vec3::from(0.0, -25.0, 1.0);

  // Light intensity proportions
  let ambient = 0.2;
  let diffuse = 0.8;

  let mut file = File::create("out.ppm").expect("Failed to create PPM file.");
  writeln!(&mut file, "P3"); 
  writeln!(&mut file, "{} {}", 801, 801);
  writeln!(&mut file, "{}", 255);

  for z in (-401..400).rev() {
    for x in -400..401 {
      let x = (x as i32 as f64)/20.0;
      let z = (z as i32 as f64)/20.0;

      let r = Ray::from(camera.clone(), Vec3::minus(&Vec3::from(x, 8.0, z), &camera));

      let mut mindist = 1_000_000.0;
      let mut pixel_colour = Vec3::from(0.0, 0.0, 0.0);

      for &s in [&s1, &s2].iter() {
        if let Some((point, len)) = s.intersect(&r) {
          if (len < mindist) {
            mindist = len;

            let normal = Vec3::minus(&point, &s.center).normalize();
            let to_light = Vec3::minus(&light, &point).normalize();
            let shade = if Vec3::dot(&normal, &to_light) < 0.0001 {
              0.0
            } else {
              Vec3::dot(&normal, &to_light)
            };

            // Scale colour by ambient and diffuse coefficients,
            // using Lambert shading for diffuse amount.
            let intensity = ambient + diffuse*shade;
            pixel_colour = s.colour.scale(intensity);
          }
        }
      }

      let red = (pixel_colour.x * 255.0) as i32;
      let green = (pixel_colour.y * 255.0) as i32;
      let blue = (pixel_colour.z * 255.0) as i32;
      write!(&mut file, "{} {} {}\t", red, green, blue);
    }
    writeln!(&mut file, "");
  }
}
