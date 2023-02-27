use nalgebra::Vector3;

use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Material: Sync {
  fn scatter(&self, ray : &Ray, record: &HitRecord, attenuation: Vector3<f32>, scattered: &Ray) -> Option<(Ray, Vector3<f32>)>;
}


pub struct Lambertian {
  albedo: Vector3<f32>,
}

impl Lambertian {
  pub fn new(albedo: Vector3<f32>) -> Lambertian {
    Self{albedo}
  }
}

impl Material for Lambertian {
  fn scatter(&self, ray : &Ray, record: &HitRecord, attenuation: Vector3<f32>, scattered: &Ray) -> Option<(Ray, Vector3<f32>)> {
    return None;
  }
}