use crate::vec3::Vec3;

pub fn write_color(pixel: Vec3) {
  println!("{} {} {}",
    (255.999 * pixel.x()) as u32,
    (255.999 * pixel.y()) as u32,
    (255.999 * pixel.z()) as u32);
}