use std::iter;
extern crate raytracer;
use raytracer::ray::Ray;
use raytracer::vec3::Vec3;

fn color(ray: &Ray) -> Vec3 {
    let unit = Vec3::unit(&ray.direction);
    let t = 0.5 * (unit.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let rows = 100_u64;
    let cols = 200_u64;
    
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let ppm_text = iter::once("P3".into())
        .chain(iter::once(format!("{} {}", cols, rows)))
        .chain(iter::once("255".into()))
        .chain((0..rows).flat_map(|row| {
            (0..cols).map(move |col| {
                let u = col as f64 / cols as f64;
                let v = row as f64 / rows as f64;
                let r = Ray::new(origin, lower_left_corner + u * horizontal + v*vertical);
                let color = color(&r);
                format!(
                    "{} {} {}",
                    (color.x() * 255.9) as usize,
                    (color.y() * 255.9) as usize,
                    (color.z() * 255.9) as usize
                )
            })
        }))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", ppm_text);
}
