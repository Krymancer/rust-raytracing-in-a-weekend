use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::point::Point;
use crate::vec3::dot;

#[derive(Copy, Clone)]
pub struct HitRecord {
  pub p : Point,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new() -> Self{
    Self{p: Point::new(0.0,0.0,0.0), normal: Point::new(0.0,0.0,0.0), t: 0.0, front_face: false}
  }

  pub fn set_face_normal(&mut self,r : &Ray, outward_normal: Vec3) {
    self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
    self.normal = if self.front_face { outward_normal } else { -outward_normal };
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
  pub fn new() -> HittableList {
    Self{objects: Vec::new()}
  }

  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::new();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for object in self.objects.iter() {
      if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;

        *hit_record = temp_rec.clone();
      }
    }

    return hit_anything;
  }
}