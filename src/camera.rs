use nalgebra::Vector3;
use crate::ray::Ray;

pub struct Camera {
  pub origin: Vector3<f32>,
  pub lower_left_corner: Vector3<f32>,
  pub horizontal: Vector3<f32>,
  pub vertical: Vector3<f32>,
}

impl Camera {
  pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, vup: Vector3<f32>, vertical_field_of_view: f32, aspect_ratio: f32) -> Self {
    let theta = vertical_field_of_view.to_radians();
    let h = (theta/2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (look_from - look_at).normalize();
    let u = vup.cross(&w).normalize();
    let v = w.cross(&u);

    let origin = look_from;
    let horizontal = viewport_width * u;
    let vertical = viewport_height * v;
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

    Self {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    return Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin);
  }
}