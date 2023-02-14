#[cfg(not(feature="rpixel"))]
use pixel_proc::dummy as extendr;
#[cfg(feature="rpixel")]
use extendr_api::prelude::*;

use super::*;
use rand::prelude::{Distribution, thread_rng};
use rand_distr::Normal;
use std::path::{PathBuf};

#[derive(Debug)]
pub struct PixelI32{
  pixel: Pixel<i32>,
}

#[allow(dead_code)]
#[extendr]
impl PixelI32 {

  pub fn new(width:i32, height:i32) -> Self {
    let p = Pixel::<i32>::create(width as usize,height as usize);
    Self {
      pixel: p,
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

  pub fn get_vec(&self) -> Vec<i32> { self.pixel.get_array().clone().to_vec() }
  
  pub fn get_index_x(&self) -> Vec<i32> {
    let mut dst = vec![0i32; self.pixel.size()];
    for y in 0..self.pixel.height() {
      for x in 0..self.pixel.width() {
        let index = x + y * self.pixel.width();
        dst[index] = x as i32;
      }
    }
    dst 
  }
  
  pub fn get_index_y(&self) -> Vec<i32> {
    let mut dst = vec![0i32; self.pixel.size()];
    for y in 0..self.pixel.height() {
      for x in 0..self.pixel.width() {
        let index = x + y * self.pixel.width();
        dst[index] = y as i32;
      }
    }
    dst 
  }

  pub fn read_file(&mut self, path: &str) {
    let path = PathBuf::from(path);
    let src = binfile_to_u8(path).unwrap();
    let offset = 64usize;
    for i in 0..self.pixel.size() {
      let buf = [src[offset + i*2+1], src[offset + i*2], 0, 0];
      self.pixel[i] = i32::from_ne_bytes(buf);
    }
  }

  pub fn read_file_with_sync(&mut self, path: &str, sync: &str) {
    let path = PathBuf::from(path);
		let vec = syncfile_to_i32(path, sync).unwrap();
    for i in 0..self.pixel.size() {
      self.pixel[i] = vec[i];
    }
  }

}


#[cfg(feature="rpixel")]
extendr_module! {
  mod pixel;
  impl PixelI32;
}


#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn it_works() {

  }
}