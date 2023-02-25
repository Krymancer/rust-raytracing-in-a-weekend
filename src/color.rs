use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel: Color, samples_per_pixel : f64) {
  let mut r = pixel.r();
  let mut g = pixel.g();
  let mut b = pixel.b();

  let scale = 1.0 / samples_per_pixel;

  r *= scale;
  g *= scale;
  b *= scale;

  println!("{} {} {}",
    (256.0 * clamp(r, 0.0, 0.999)) as u32,
    (256.0 * clamp(g, 0.0, 0.999)) as u32,
    (256.0 * clamp(b, 0.0, 0.999)) as u32);
}

fn clamp(x : f64, min: f64, max: f64) -> f64 {
  if x < min { return min };
  if x > max { return max };
  return x;
}
