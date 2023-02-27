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

pub const SAMPLES_PER_PIXEL : u32 = 500;
pub const WIDTH : u32 = 1200;
pub const MAX_DEPTH : u32 = 50;

fn main() {
    // Image
    let aspect_ratio: f32 = 3.0 / 2.0;
    let width: u32 = WIDTH;
    let height: u32 = ((width as f32)/aspect_ratio) as u32;

    // World 
    let world = random_scene();

    // Camera
    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let vertical_field_of_view = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera : Camera = Camera::new(look_from, look_at, vup, vertical_field_of_view, aspect_ratio, aperture, dist_to_focus);

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


fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world = HittableList::new();
    world.add(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(Vector3::new(0.5, 0.5, 0.5))));
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 { // diffuse
                    world.add(
                        Sphere::new(center, 0.2,
                                    Lambertian::new(Vector3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>()))));
                } else if choose_material < 0.95 { // metal
                    world.add(
                        Sphere::new(center, 0.2,
                                    Metal::new(Vector3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())), 0.5 * rng.gen::<f32>())));
                } else { // glass
                    world.add( Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }
    world.add(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    world.add(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Vector3::new(0.4, 0.2, 0.1))));
    world.add(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)));
    return world
}