use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel: Color) {
  println!("{} {} {}",
    (255.999 * pixel.x()) as u32,
    (255.999 * pixel.y()) as u32,
    (255.999 * pixel.z()) as u32);
}