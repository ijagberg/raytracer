use std::iter;
extern crate raytracer;
use raytracer::ray::Ray;
use raytracer::vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction * ray.direction;
    let b = 2.0 * oc * ray.direction;
    let c = oc * oc - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(ray: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = Vec3::unit(&(ray.point_at(t) - Vec3::new(0.0, 0.0, -1.0)));
        0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    } else {
        let unit_direction = Vec3::unit(&ray.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
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
        .chain((0..rows).rev().flat_map(|row| {
            (0..cols).map(move |col| {
                let u = col as f64 / cols as f64;
                let v = row as f64 / rows as f64;
                let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
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
