mod ray;
mod color;
mod hittable;
mod sphere;
mod camera;
mod random;
mod material;
mod hit_record;

use nalgebra::Vector3;
use crate::sphere::Sphere;
use crate::hittable::HittableList;
use crate::camera::Camera;
use crate::ray::ray_color;
use crate::random::random;
use crate::material::Lambertian;

pub const SAMPLES_PER_PIXEL : u32 = 100;
pub const WIDTH : u32 = 200;
pub const MAX_DEPTH : u32 = 50;

fn main() {
    // Image
    let aspect_ratio : f64 = 16.0 / 9.0;
    let width: u32 = WIDTH;
    let height: u32 = ((width as f64)/aspect_ratio) as u32;

    // World 
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Vector3::new(0.0,0.0,0.0)))));
    world.add(Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Vector3::new(0.0,0.0,0.0)))));
    
    // Camera
    let camera : Camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut pixel_color : Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u : f32 = ((i as f32) + random()) / ((width - 1) as f32);
                let v : f32 = ((j as f32) + random()) / ((height - 1) as f32);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            color::write_color(pixel_color, SAMPLES_PER_PIXEL as f32);
        }
    }
}


