use nalgebra::Vector3;
use rand::Rng;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Material: Sync {
  fn scatter(&self, ray : &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
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
  fn scatter(&self, _ : &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
    let scatter_direction = record.normal + Vector3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()).normalize();
    let scattered  = Ray::new(record.p, scatter_direction);
    return Some((scattered, self.albedo));
  }
}

pub struct Metal {
  albedo: Vector3<f32>,
  fuzz: f32,
}

impl Metal {
  pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
    Self { albedo, fuzz: if fuzz < 1.0 {fuzz} else {1.0}}
  }
}

impl Material for Metal {
  fn scatter(&self, ray : &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
      let mut reflected = reflect(ray.direction().normalize(), record.normal);
      if self.fuzz > 0.0 { reflected += self.fuzz * random_in_unit_sphere()}
      
      if reflected.dot(&record.normal) > 0.0 {
        let scattered = Ray::new(record.p, reflected);
        return Some((scattered, self.albedo));
      } else { 
        return None;
      }

  }
}

fn random_in_unit_sphere() -> Vector3<f32> {
  let mut rng = rand::thread_rng();
  let unit = Vector3::new(1.0, 1.0, 1.0);
  loop {
      let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - unit;
      if p.magnitude_squared() < 1.0 {
          return p
      }
  }
}

fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
  return v - 2.0 * v.dot(&n) * n;
}

fn refract(v: &Vector3<f32>, n: &Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
  let uv = v.normalize();
  let dt = uv.dot(&n);
  let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
  if discriminant > 0.0 {
      let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
      Some(refracted)
  } else {
      None
  }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
  let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
  r0 + (1.0 -r0) * (1.0 - cosine).powi(5)
}