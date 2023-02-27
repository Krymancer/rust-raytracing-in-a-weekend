use nalgebra::Vector3;

pub fn write_color(pixel: Vector3<f32>, samples_per_pixel : f32) {
  let mut r = pixel[0];
  let mut g = pixel[1];
  let mut b = pixel[2];

  let scale = 1.0 / samples_per_pixel;

  r = (r * scale).sqrt();
  g = (g * scale).sqrt();
  b = (b * scale).sqrt();

  println!("{} {} {}",
    (256.0 * clamp(r, 0.0, 0.999)) as u32,
    (256.0 * clamp(g, 0.0, 0.999)) as u32,
    (256.0 * clamp(b, 0.0, 0.999)) as u32);
}

fn clamp(x : f32, min: f32, max: f32) -> f32 {
  if x < min { return min };
  if x > max { return max };
  return x;
}
