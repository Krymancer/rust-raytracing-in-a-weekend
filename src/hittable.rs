use crate::ray::Ray;
use crate::hit_record::HitRecord;

pub trait Hittable : Sync {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
  pub fn new() -> HittableList {
    Self{objects: Vec::new()}
  }

  pub fn add(&mut self, object: impl Hittable + 'static) {
    self.objects.push(Box::new(object));
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut hit_anything : Option<HitRecord> = None;
    let mut closest_so_far = t_max;
    
    for object in self.objects.iter() {
      if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
        closest_so_far = hit.t;
        hit_anything = Some(hit);
      }
    }

    return hit_anything;
  }
}