use crate::hittable::Hittable;
use crate::point::Point;
use crate::vec3::Vec3;
use crate::vec3::dot;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct Sphere {
  center: Point,
  radius: f64,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64) -> Sphere {
    Self{center, radius}
  }
}

impl Hittable for Sphere{
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
    let oc = r.origin() - self.center;
    let a = r.direction().squared_length();
    let half_b = dot(&oc, &r.direction()); //let b = 2.0 * dot(&oc, &r.direction());
    let c = oc.squared_length() - self.radius * self.radius;
    let discriminant = half_b * half_b - a*c;
    if discriminant < 0.0 { return false; }
    let sqrtd = discriminant.sqrt();
    
    // Find the nearest root that lies in the acceptable range.
    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return false;
      }
    }

    hit_record.t = root;
    hit_record.p = r.point_at_parameter(hit_record.t);
    let outward_normal = (hit_record.p - self.center) / self.radius;
    hit_record.set_face_normal(r, outward_normal);

    return true;
  }
}
