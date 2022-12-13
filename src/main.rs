mod vec3;
fn main() {
    let width: u32 = 200;
    let height: u32 = 100;

    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let col : vec3::Vec3 = vec3::Vec3{x: i as f32 / width as f32, y: j as f32 / height as f32, z: 0.2};

            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}


