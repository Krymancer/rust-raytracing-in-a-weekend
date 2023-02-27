use nalgebra::Vector3;
use crate::ray::Ray;

pub struct Camera {
  pub origin: Vector3<f32>,
  pub lower_left_corner: Vector3<f32>,
  pub horizontal: Vector3<f32>,
  pub vertical: Vector3<f32>,
}

impl Camera {
  pub fn new() -> Self {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3::new(0.0, 0.0, focal_length);

    Self {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }

  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    return Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin);
  }
}