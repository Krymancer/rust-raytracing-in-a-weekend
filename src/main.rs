mod vec3;
mod ray;
mod color;

use ray::Ray;
use vec3::Vec3;

fn ray_color(r: Ray) -> Vec3 {
    let unit_direction: Vec3 = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width: u32 = 400;
    let height: u32 = ((width as f32)/aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f32 / (width - 1) as f32;
            let v = j as f32 / (height - 1) as f32;

            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let col = ray_color(r);
            color::write_color(col);
        }
    }
}


