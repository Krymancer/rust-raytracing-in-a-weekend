mod vec3;
mod ray;
mod color;
mod point;
mod hittable;
mod util;
mod sphere;
mod camera;
mod random;

use crate::color::Color;
use crate::vec3::Vec3;
use crate::sphere::Sphere;
use crate::hittable::HittableList;
use crate::camera::Camera;
use crate::ray::ray_color;
use crate::random::random;

pub const SAMPLES_PER_PIXEL : u64 = 100;
pub const WIDTH : u64 = 200;
pub const MAX_DEPTH : u64 = 50;

fn main() {
    // Image
    let aspect_ratio : f64 = 16.0 / 9.0;
    let width: u64 = WIDTH;
    let height: u64 = ((width as f64)/aspect_ratio) as u64;

    // World 
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    
    // Camera
    let camera : Camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut pixel_color : Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u : f64 = ((i as f64) + random()) / ((width - 1) as f64);
                let v : f64 = ((j as f64) + random()) / ((height - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            color::write_color(pixel_color, SAMPLES_PER_PIXEL as f64);
        }
    }
}


