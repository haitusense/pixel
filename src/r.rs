#[cfg(not(feature="rpixel"))]
use pixel_proc::dummy as extendr;
#[cfg(feature="rpixel")]
use extendr_api::prelude::*;

use super::*;
use rand::prelude::{Distribution, thread_rng};
use rand_distr::Normal;

#[extendr]
pub fn rpixel_hello() -> &'static str {
  "Hello rpixel!"
}

#[derive(Debug)]
pub struct PixelI32{
  pixel: Pixel<i32>,
  index_x : Vec<i32>,
  index_y : Vec<i32>,
}

#[allow(dead_code)]
#[extendr]
impl PixelI32 {

  pub fn new(width:i32, height:i32) -> Self {
    let p = Pixel::<i32>::create(width as usize,height as usize);
    let mut w = vec![0i32; (width * height) as usize];
    let mut h = vec![0i32;  (width * height) as usize];
    for y in 0..height {
      for x in 0..width {
        let index = (x + y * width) as usize;
        w[index] = x;
        h[index] = y;
      }
    }
    Self {
      pixel: p,
      index_x : w,
      index_y : h,
    }
  }

  pub fn clear_rnd(&mut self) {

    let mut rng = thread_rng();
    let dist = Normal::<f64>::new(125.0, 10.0).unwrap();
    
    for i in 0..self.pixel.size() {
      self.pixel[i] = dist.sample(&mut rng) as i32;
    }
  
  }

  pub fn width(&self) -> i32 { self.pixel.width() as i32 }
  pub fn height(&self) -> i32 { self.pixel.height() as i32 }

  pub fn get_c_v(&self) -> Vec<i32> { self.pixel.get_array().clone().to_vec() }
  pub fn get_c_x(&self) -> Vec<i32> { self.index_x.clone() }
  pub fn get_c_y(&self) -> Vec<i32> { self.index_y.clone() }

}



#[cfg(feature="rpixel")]
extendr_module! {
  mod pixel;
  fn rpixel_hello;
  impl PixelI32;
}

// #[cfg(not(feature="rpixel"))]
// #[cfg(test)]
// mod tests {
//   use crate::*;

//   #[test]
//   fn it_works() {
//     println!("{}",rpixel_hello());
//   }
// }