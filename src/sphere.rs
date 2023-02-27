use nalgebra::Vector3;

use crate::hittable::Hittable;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere<M: Material> {
  center: Vector3<f32>,
  radius: f32,
  material: M,
}

impl<M: Material> Sphere<M> {
  pub fn new(center: Vector3<f32>, radius: f32, material: M) -> Sphere<M> {
    Self{center, radius, material}
  }
}

impl<M: Material> Hittable for Sphere<M>{
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) ->  Option<HitRecord> {
    let oc = ray.origin() - self.center;
    let a = ray.direction().dot(&ray.direction());
    let half_b = oc.dot(&ray.direction());
    let c = oc.dot(&oc) - self.radius * self.radius;
    let discriminant = half_b * half_b - a*c;
    if discriminant < 0.0 { return None; }

    let sqrtd = discriminant.sqrt();
    
    // Find the nearest root that lies in the acceptable range.
    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return None;
      }
    }

    let t = root;
    let p = ray.point_at_parameter(root);
    let normal = (p - self.center) / self.radius;

    return Some(HitRecord { t, p, normal, material: &self.material});
  }
}
