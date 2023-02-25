use rand::Rng;

pub fn random() -> f64 {
  // Generate a random number in [0,1)
  let mut rng = rand::thread_rng();
  rng.gen()
}

// pub fn random_range(min: f64, max: f64) -> f64{
//   // Generate a random number in [min,max)
//   min + (max - min) * random()
// }