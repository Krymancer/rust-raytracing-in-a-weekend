use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;

pub struct Camera {
  origin: Vector3<f32>,
  lower_left_corner: Vector3<f32>,
  horizontal: Vector3<f32>,
  vertical: Vector3<f32>,
  v: Vector3<f32>,
  u: Vector3<f32>,
  lens_radius: f32 
}

impl Camera {
  pub fn new(
      look_from: Vector3<f32>,
      look_at: Vector3<f32>,
      vup: Vector3<f32>,
      vertical_field_of_view: f32, 
      aspect_ratio: f32,
      aperture: f32,
      focus_distance: f32
  ) -> Self {
    let theta = vertical_field_of_view.to_radians();
    let h = (theta/2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (look_from - look_at).normalize();
    let u = vup.cross(&w).normalize();
    let v = w.cross(&u);

    let origin = look_from;
    let horizontal = focus_distance * viewport_width * u;
    let vertical = focus_distance * viewport_height * v;
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_distance*w;
    let lens_radius = aperture / 2.0;

    Self {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
      v,
      u,
      lens_radius
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = self.u * rd[0] + self.v * rd[1];
    return Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset);
  }
}

fn random_in_unit_disk() -> Vector3<f32> {
  let mut rng = rand::thread_rng();
  let unit = Vector3::new(1.0, 1.0, 0.0);
  loop {
      let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - unit;
      if p.dot(&p) < 1.0 {
          return p
      }
  }
}