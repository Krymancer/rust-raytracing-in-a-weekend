use rand::Rng;

pub fn random() -> f32 {
  // Generate a random number in [0,1)
  let mut rng = rand::thread_rng();
  rng.gen()
}

pub fn random_range(min: f32, max: f32) -> f32{
  // Generate a random number in [min,max)
  min + (max - min) * random()
}