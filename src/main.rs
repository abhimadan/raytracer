mod geometry;

use geometry::Vec3;
use geometry::Ray;
use geometry::Sphere;
use std::fs::File;
use std::io::Write;

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
    let _ = writeln!(&mut file, "P3");
    let _ = writeln!(&mut file, "{} {}", 401, 401);
    let _ = writeln!(&mut file, "{}", 255);

    for z in (-200..201).rev() {
        for x in -200..201 {
            let x_center = (x as i32 as f64)/10.0;
            let z_center = (z as i32 as f64)/10.0;

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for dx in [-0.033, 0.0, 0.033].iter() {
                for dz in [-0.033, 0.0, 0.033].iter() {
                    let x = x_center + dx;
                    let z = z_center + dz;

                    let r = Ray::from(camera.clone(), Vec3::minus(&Vec3::from(x, 8.0, z), &camera));

                    let mut mindist = 1_000_000.0;
                    let mut pixel_colour = Vec3::from(0.0, 0.0, 0.0);

                    for &s in [&s1, &s2].iter() {
                        if let Some((point, len)) = s.intersect(&r) {
                            if len < mindist {
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

                    red += (pixel_colour.x * 255.0) as i32;
                    green += (pixel_colour.y * 255.0) as i32;
                    blue += (pixel_colour.z * 255.0) as i32;
                }
            }

            red /= 9;
            green /= 9;
            blue /= 9;
            let _ = write!(&mut file, "{} {} {}\t", red, green, blue);
        }
        let _ = writeln!(&mut file, "");
    }
}
