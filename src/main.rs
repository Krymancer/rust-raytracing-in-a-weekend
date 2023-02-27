mod ray;
mod color;
mod hittable;
mod sphere;
mod camera;
mod random;
mod material;
mod hit_record;

use rand::Rng;
use nalgebra::Vector3;

use crate::sphere::Sphere;
use crate::hittable::HittableList;
use crate::camera::Camera;
use crate::ray::ray_color;
use crate::material::{Lambertian, Metal};

pub const SAMPLES_PER_PIXEL : u32 = 100;
pub const WIDTH : u32 = 400;
pub const MAX_DEPTH : u32 = 50;

fn main() {
    // Image
    let aspect_ratio : f64 = 16.0 / 9.0;
    let width: u32 = WIDTH;
    let height: u32 = ((width as f64)/aspect_ratio) as u32;

    // World 
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Vector3::new(0.8, 0.8, 0.0));
    let center_material = Lambertian::new(Vector3::new(0.7, 0.3, 0.3));
    let left_material = Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.3);
    let right_material = Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(Vector3::new( 0.0, -100.5, -1.0), 100.0, ground_material));
    world.add(Sphere::new(Vector3::new( 0.0,    0.0, -1.0),   0.5, center_material));
    world.add(Sphere::new(Vector3::new(-1.0,    0.0, -1.0),   0.5, left_material));
    world.add(Sphere::new(Vector3::new( 1.0,    0.0, -1.0),   0.5, right_material));
    
    // Camera
    let camera : Camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", width, height);

    let image = (0..height).into_iter().rev().flat_map(|y| 
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

    // for j in (0..height).rev() {
    //     for i in 0..width {
    //         let mut pixel_color : Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    //         for _ in 0..SAMPLES_PER_PIXEL {
    //             let mut rng = rand::thread_rng();
    //             let u : f32 = ((i as f32) + rng.gen::<f32>()) / ((width - 1) as f32);
    //             let v : f32 = ((j as f32) + rng.gen::<f32>()) / ((height - 1) as f32);
    //             let ray = camera.get_ray(u, v);
    //             pixel_color += ray_color(&ray, &world, MAX_DEPTH);
    //         }
    //         color::write_color(pixel_color, SAMPLES_PER_PIXEL as f32);
    //     }
    // }
}


