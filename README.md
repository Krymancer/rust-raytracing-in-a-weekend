# Rust Raytracer

This is a Rust implementation of a raytracer based on the "Ray Tracing in One Weekend" book by Peter Shirley. The goal of this project is to learn Rust by implementing a simple raytracer.

## Features

- Basic raytracing algorithm with spheres and planes.
- ~~Antialiasing using random sampling.~~ (not implemented yet)
- ~~Diffuse, metallic and dielectric materials.~~ (not implemented yet)
- ~~Multi-threaded rendering using Rayon.~~ (not implemented yet)

## Usage

To run the raytracer, execute the following command

```bash
cargo run --release
```

The output image will be saved to the file output.ppm in the project directory.

## Configuration

The raytracer can be configured by editing the variables in the `main` function in the `src/main.rs` file.
Changing the width automatically changes the height of the image to keep the aspect ratio. (The base aspect ratio is 16/9)

```rust
let width = 400;
```

## Scene

To modify the scene, edit the `world` variable:

```rust
  // World 
  let mut world = HittableList::new();
  world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
  world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
```

The scene is defined as a list of `Hittable` objects. Currently, the scene contains spheres ~~and a plane~~. You can add, remove or modify these objects to create your own scene.

Each object is defined by its position and radius ~~(for spheres) or distance (for planes), and material. The available materials are Lambertian (diffuse), Metal (reflective) and Dielectric (refractive).~~

## ~~Anti-aliasing~~

~~To change the level of antialiasing, edit the `samples_per_pixel` variable:~~

```rust
let samples_per_pixel = 100;
```

Higher values of `samples_per_pixel´ will result in a smoother image but will also increase rendering time.

## ~~Multi-threading~~

~~To change the number of threads used for rendering, edit the `num_threads` variable:~~

```rust
let num_threads = 8;
```
