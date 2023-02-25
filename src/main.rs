mod vec3;
mod ray;
mod color;
mod point;
mod hittable;
mod util;
mod sphere;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::Hittable;
use crate::sphere::Sphere;
use crate::hittable::HittableList;

fn ray_color(r: Ray, world: &HittableList) -> Vec3 {
    let mut hit_record : HitRecord = HitRecord::new();
    if world.hit(&r, 0.0, util::INFINITY, &mut hit_record) {
        let color = 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
        return color;
    }

    let unit_direction = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0-t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio : f64 = 16.0 / 9.0;
    let width: u64 = 400;
    let height: u64 = ((width as f64)/aspect_ratio) as u64;

    // World 
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64= aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;

            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let col = ray_color(r, &world);
            color::write_color(col);
        }
    }
}


