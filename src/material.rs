use nalgebra::Vector3;
use rand::Rng;

use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Material: Sync {
  fn scatter(&self, ray : &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
      let mut reflected = reflect(&ray.direction().normalize(), &record.normal);
      if self.fuzz > 0.0 { reflected += self.fuzz * random_in_unit_sphere()}
      
      if reflected.dot(&record.normal) > 0.0 {
        let scattered = Ray::new(record.p, reflected);
        return Some((scattered, self.albedo));
      } else { 
        return None;
      }

  }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
  ir: f32,
}

impl Dielectric {
  pub fn new(ir: f32) -> Self {
    Self { ir }
  }
}


impl Material for Dielectric {
  fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
    let attenuation = Vector3::new(1.0, 1.0, 1.0);
    let hit_angle = ray.direction().dot(&record.normal);

    let (outward_normal, etai_over_etat, cosine) = if hit_angle > 0.0 {
      let cosine = self.ir * ray.direction().dot(&record.normal) / ray.direction().magnitude();
      (-record.normal, self.ir, cosine)
    } else {
        let cosine = -ray.direction().dot(&record.normal) / ray.direction().magnitude();
        (record.normal, 1.0 / self.ir, cosine)
    };

    if let Some(refracted) = refract(&ray.direction(), &outward_normal, etai_over_etat) {
      let reflect_prob = schlick(cosine, self.ir);
      if rand::thread_rng().gen::<f32>() >= reflect_prob {
        let scattered  = Ray::new(record.p, refracted);
        return Some((scattered, attenuation));
      }
    }

    let reflected = reflect(&ray.direction(), &record.normal);
    let scattered  = Ray::new(record.p, reflected);
    return Some((scattered, attenuation));
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

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
  return v - 2.0 * v.dot(&n) * n;
}

fn refract(v: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Option<Vector3<f32>> {
  let uv = v.normalize(); // In the book the input is normalized in the caller function, but I think it's better to do it here
  let dt = uv.dot(&n); // Angle between the ray and the normal
  let discriminant = 1.0 - etai_over_etat.powi(2) * (1.0 - dt.powi(2)); // Using law of cosines and the Snell's law
  if discriminant > 0.0 {
      // Can refract
      let refracted = etai_over_etat * (uv - n * dt) - n * discriminant.sqrt();
      Some(refracted)
  } else {
      // Must reflect
      None
  }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
  let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
  r0 + (1.0 -r0) * (1.0 - cosine).powi(5)
}