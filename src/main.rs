mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;
mod hit_record;

use rand::Rng;
use nalgebra::Vector3;

use crate::sphere::Sphere;
use crate::hittable::HittableList;
use crate::camera::Camera;
use crate::ray::ray_color;
use crate::material::{Lambertian, Metal, Dielectric};

use rayon::prelude::*;

pub const SAMPLES_PER_PIXEL : u32 = 100;
pub const WIDTH : u32 = 800;
pub const MAX_DEPTH : u32 = 50;

fn main() {
    // Image
    let aspect_ratio : f64 = 16.0 / 9.0;
    let width: u32 = WIDTH;
    let height: u32 = ((width as f64)/aspect_ratio) as u32;

    // World 
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Vector3::new(0.8, 0.8, 0.0));
    let center_material = Lambertian::new(Vector3::new(0.1, 0.2, 0.5));
    let left_material = Dielectric::new(1.5);
    let left_material2 = Dielectric::new(1.5);
    let right_material = Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.0);

    world.add(Sphere::new(Vector3::new( 0.0, -100.5, -1.0), 100.0, ground_material));
    world.add(Sphere::new(Vector3::new( 0.0,    0.0, -1.0),   0.5, center_material));
    world.add(Sphere::new(Vector3::new(-1.0,    0.0, -1.0),   0.5, left_material));
    world.add(Sphere::new(Vector3::new(-1.0,    0.0, -1.0),   -0.4, left_material2));
    world.add(Sphere::new(Vector3::new( 1.0,    0.0, -1.0),   0.5, right_material));
    
    // Camera
    let camera : Camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", width, height);

    let image = (0..height).into_par_iter().rev().flat_map(|y| 
        (0..width).flat_map(|x| {
            let color : Vector3<f32> = (0..SAMPLES_PER_PIXEL).map(|_| {
                let mut rng = rand::thread_rng();
                let u : f32 = ((x as f32) + rng.gen::<f32>()) / ((width - 1) as f32);
                let v : f32 = ((y as f32) + rng.gen::<f32>()) / ((height - 1) as f32);
                let ray = camera.get_ray(u, v);
                return ray_color(&ray, &world, MAX_DEPTH);
            }).sum();
            color.iter().map(|c| (255.99 * (c/(SAMPLES_PER_PIXEL as f32)).sqrt().max(0.0).min(1.0)) as u8).collect::<Vec<u8>>()
        }).collect::<Vec<u8>>()
    ).collect::<Vec<u8>>();

    for color in image.chunks(3) {
        println!("{} {} {}", color[0], color[1], color[2]);
    }
}


