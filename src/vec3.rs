use std::ops;
pub struct Vec3{
  pub x : f32,
  pub y : f32,
  pub z : f32
}

impl Vec3 {
  pub fn new(&self, x : f32, y : f32, z : f32) -> Self{
    Vec3{x, y, z}
  }

  pub fn new_blank() -> Self {
    Vec3{x: 0.0, y: 0.0, z: 0.0}
  }

  pub fn x(&self) -> f32 {
    self.x
  }

  pub fn y(&self) -> f32 {
    self.y
  }

  pub fn z(&self) -> f32 {
    self.z
  }

  pub fn r(&self) -> f32 {
    self.x
  }

  pub fn g(&self) -> f32 {
    self.y
  } 

  pub fn b(&self) -> f32 {
    self.z
  }

  pub fn length(&self) -> f32 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  pub fn squared_length(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn make_unit_vector(&self) -> Self {
    let k = 1.0 / self.length();
    Vec3{x: self.x * k, y: self.y * k, z: self.z * k}
  }

  pub fn dot(&self, v : &Vec3) -> f32 {
    self.x * v.x + self.y * v.y + self.z * v.z
  }

  pub fn cross(&self, v : &Vec3) -> Self {
    Vec3{x: self.y * v.z - self.z * v.y,
         y: self.z * v.x - self.x * v.z,
         z: self.x * v.y - self.y * v.x}
  }

  pub fn unit_vector(&self) -> Self {
    Vec3 {
      x: self.x / self.length(),
      y: self.y / self.length(),
      z: self.z / self.length()
    }
  }
}

impl ops::Add<Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs : Vec3) -> Vec3 {
    Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
  }
}

impl ops::Sub<Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs : Vec3) -> Vec3 {
    Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
  }
}

impl ops::Mul<Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs : Vec3) -> Vec3 {
    Vec3{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
  }
}

impl ops::Mul<f32> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs : f32) -> Vec3 {
    Vec3{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
  }
}

impl ops::Div<f32> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs : f32) -> Vec3 {
    Vec3{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
  }
}

impl ops::Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Vec3 {
    Vec3{x: -self.x, y: -self.y, z: -self.z}
  }
}

impl ops::Index<usize> for Vec3 {
  type Output = f32;

  fn index(&self, i : usize) -> &f32 {
    match i {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Index out of bounds")
    }
  }
}

impl ops::IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, i : usize) -> &mut f32 {
    match i {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => panic!("Index out of bounds")
    }
  }
}

impl ops::AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, rhs : Vec3) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
  }
}

impl ops::SubAssign<Vec3> for Vec3 {
  fn sub_assign(&mut self, rhs : Vec3) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
  }
}

impl ops::MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, rhs : f32) {
    self.x *= rhs;
    self.y *= rhs;
    self.z *= rhs;
  }
}

impl ops::DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, rhs : f32) {
    self.x /= rhs;
    self.y /= rhs;
    self.z /= rhs;
  }
}

impl ops::MulAssign<Vec3> for Vec3 {
  fn mul_assign(&mut self, rhs : Vec3) {
    self.x *= rhs.x;
    self.y *= rhs.y;
    self.z *= rhs.z;
  }
}

impl ops::DivAssign<Vec3> for Vec3 {
  fn div_assign(&mut self, rhs : Vec3) {
    self.x /= rhs.x;
    self.y /= rhs.y;
    self.z /= rhs.z;
  }
}

impl ops::Add<f32> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs : f32) -> Vec3 {
    Vec3{x: self.x + rhs, y: self.y + rhs, z: self.z + rhs}
  }
}

impl ops::Sub<f32> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs : f32) -> Vec3 {
    Vec3{x: self.x - rhs, y: self.y - rhs, z: self.z - rhs}
  }
}

impl ops::AddAssign<f32> for Vec3 {
  fn add_assign(&mut self, rhs : f32) {
    self.x += rhs;
    self.y += rhs;
    self.z += rhs;
  }
}

impl ops::SubAssign<f32> for Vec3 {
  fn sub_assign(&mut self, rhs : f32) {
    self.x -= rhs;
    self.y -= rhs;
    self.z -= rhs;
  }
}

impl ops::Add<Vec3> for f32 {
  type Output = Vec3;

  fn add(self, rhs : Vec3) -> Vec3 {
    Vec3{x: self + rhs.x, y: self + rhs.y, z: self + rhs.z}
  }
}

impl ops::Sub<Vec3> for f32 {
  type Output = Vec3;

  fn sub(self, rhs : Vec3) -> Vec3 {
    Vec3{x: self - rhs.x, y: self - rhs.y, z: self - rhs.z}
  }
}

