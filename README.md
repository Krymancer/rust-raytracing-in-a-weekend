# Rust Raytracer

This is a Rust implementation of a raytracer based on the "Ray Tracing in One Weekend" book by Peter Shirley. The goal of this project is to learn Rust by implementing a simple raytracer.

## Features

- Basic raytracing algorithm with spheres and planes.
- Antialiasing using random sampling.
- Diffuse, metallic and dielectric materials
- Multi-threaded rendering using Rayon.

## Usage

To run the raytracer, execute the following command

```bash
cargo run --release
```

The output is a [PPM](https://netpbm.sourceforge.net/doc/ppm.html) image. you can save the output of the program to a file and open it with any image viewer that supports ppm.
I recommend [GIMP](https://www.gimp.org/).
Or you can use [this ppm viewer](https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html) to view the image in your browser.

## Output

This is mimic the output of the original C++ implementation of the book.
![output]([output.png](https://raw.githubusercontent.com/Krymancer/rust-raytracing-in-a-weekend/main/.github/image.png))

## Configuration

The raytracer can be configured by editing the variables in the `main` function in the `src/main.rs` file.
Changing the `WIDTH` automatically changes the height of the image to keep the aspect ratio. (The base aspect ratio is 16/9)

```rust
pub const WIDTH : u32 = 400;
```

## Scene

To modify the scene, edit the `world` variable:

```rust
let mut world = HittableList::new();
world.add(Sphere::new(Vector3::new( 0.0, -100.5, -1.0), 100.0, Lambertian::new(Vector3::new(0.8, 0.8, 0.0))));
world.add(Sphere::new(Vector3::new( 0.0,    0.0, -1.0),   0.5, Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
world.add(Sphere::new(Vector3::new(-1.0,    0.0, -1.0),   0.5, Dielectric::new(1.5)));
world.add(Sphere::new(Vector3::new( 1.0,    0.0, -1.0),   0.5, Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.0)));
```

The scene is defined as a list of `Hittable` objects. Currently, the scene contains spheres ~~and a plane~~. You can add, remove or modify these objects to create your own scene.

Each object is defined by its position, radius, and material. The available materials are Lambertian (diffuse), Metal (reflective) and Dielectric (refractive).

## Anti-aliasing

To change the level of antialiasing, edit the `SAMPLES_PER_PIXEL` variable:

```rust
pub const SAMPLES_PER_PIXEL : u32 = 100;
```

Higher values of `samples_per_pixel` will result in a smoother image but will also increase rendering time.

## Multi-threading

I used [rayon](https://github.com/rayon-rs/rayon) to active multi-threading.
