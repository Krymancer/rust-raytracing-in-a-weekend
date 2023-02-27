use nalgebra::Vector3;

use crate::hittable::HittableList;
use crate::hittable::Hittable;

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }
}

pub fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Vector3<f32> {
    if let Some(hit) = world.hit(&ray, 0.001, f32::MAX) {

        if depth <= 0 {
            return Vector3::new(0.0,0.0,0.0);
        }    

        if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
            return attenuation.zip_map(&ray_color(&scattered, &world, depth-1), |l, r| l * r);
        } else {
            return Vector3::new(0.0,0.0,0.0);
        }
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        return (1.0-t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}