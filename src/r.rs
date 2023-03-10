#[cfg(not(feature="rpixel"))]
use pixel_proc::dummy as extendr;
#[cfg(feature="rpixel")]
use extendr_api::prelude::*;

#[allow(unused_imports)]
use polars::prelude::*;

#[allow(unused_imports)]
use crate::log::*;

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
  
  pub fn get_index(&self, option: &str) -> Vec<i32> {
    let mut dst = vec![0i32; self.pixel.size()];
    match option {
      "x" => {
        for y in 0..self.pixel.height() {
          for x in 0..self.pixel.width() {
            let index = x + y * self.pixel.width();
            dst[index] = x as i32;
          }
        }
      },
      "y" => {
        for y in 0..self.pixel.height() {
          for x in 0..self.pixel.width() {
            let index = x + y * self.pixel.width();
            dst[index] = y as i32;
          }
        }
      },
      "cimg_x" => {
        for y in 0..self.pixel.height() {
          for x in 0..self.pixel.width() {
            let index = x + y * self.pixel.width();
            dst[index] = (x + 1) as i32;
          }
        }
      },
      "cimg_y" => {
        for y in 0..self.pixel.height() {
          for x in 0..self.pixel.width() {
            let index = x + y * self.pixel.width();
            dst[index] = (y + 1) as i32;
          }
        }
      },
      "bayer" => {
        for y in 0..self.pixel.height() {
          for x in 0..self.pixel.width() {
            let index = x + y * self.pixel.width();
            dst[index] = (x % 2 + (y % 2) * 2)  as i32;
          }
        }
      }
      _=>{ }
    }
    dst 
  }
  
  #[cfg(feature="rpixel")]
  pub fn get_df(&self) -> Robj {

    let mut dst_x = vec![0i32; self.pixel.size()];
    let mut dst_y = vec![0i32; self.pixel.size()];
    let mut dst_bayer = vec![0i32; self.pixel.size()];
    let value = self.pixel.get_array().clone().to_vec();
   
    let mut dst = vec![0i32; self.pixel.size()];
    for y in 0..self.pixel.height() {
      for x in 0..self.pixel.width() {
        let index = x + y * self.pixel.width();
        dst_x[index] = x as i32 + 1;
        dst_y[index] = y as i32 + 1;
        dst_bayer[index] = (x % 2 + (y % 2) * 2)  as i32;
      }
    }

    data_frame!(x=dst_x, y=dst_y, bayer=dst_bayer, value=value) 
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


#[allow(dead_code)]
#[derive(Debug)]
pub struct RpxLog{
  src: String,
}

#[allow(dead_code)]
#[extendr]
impl RpxLog {

  pub fn new(path: &str) -> Self {
    Self {
      src: log::read_logfile(path).unwrap(),
    }
  }

  #[cfg(feature="rpixel")]
  pub fn header_to_df(&self) -> Robj {
    let df = log::logheader_to_vec(&self.src).unwrap();
    data_frame!(key=df.0, value=df.1) 
  }

  #[cfg(feature="rpixel")]
  pub fn body_to_df(&self) -> Robj {
    let df = log::logbody_to_vec(&self.src).unwrap();
    data_frame!(
      tcnt=df.tcnt,
      site=df.site,
      test_index=df.test_index,
      key=df.key,
      signal=df.signal,
      value=df.value
    ) 
  }
  
}

#[cfg(feature="rpixel")]
#[extendr]
pub fn value_with_unit(src:&str) -> Rfloat {
  match log::unit::value_with_unit_to_f64(src) {
    Ok(n) => Rfloat::from(n),
    Err(_) => Rfloat::na()
  }
}

#[cfg(feature="rpixel")]
#[extendr]
pub fn polars_test() -> Robj {
  let mut df1: DataFrame = df!(
    "Fruit" => &["Apple", "Banana", "Pear"],
    "Origin" => &["America", "Hawai", "Italy"],
    "Phosphorus (mg/100g)" => &[11, 22, 12]).unwrap();
  let df2: DataFrame = df!(
    "Name" => &["Apple", "Banana", "Pear"],
    "Origin" => &["France", "Hawai", "Italy"],
    "Potassium (mg/100g)" => &[107, 358, 115]).unwrap();
  
  let mut dst = df1.join(&df2, ["Fruit"], ["Name"], JoinType::Inner, None).unwrap();

  println!("{dst:?}");

  let h = &dst.get_column_names()
  .iter()
  // .filter(|x| x.starts_with("A"))
  .map(|&x| x)
  .collect::<Vec<&str>>();
  println!("{h:?}");
  
  let h = &dst.get_columns()[2]
  .iter()
  // .filter(|x| x.starts_with("A"))
  .map(|n| match n {
    AnyValue::Int32(n) => n,
    _ => 0
   } )
  .collect::<Vec<i32>>();
  println!("{h:?}");

  let h2 = &dst.get_columns()[2]
  .iter()
  .map(|n| format!("{}",n))
  .collect::<Vec<String>>();

  let h0 = &dst.get_columns()[0]
  .iter()
  .map(|n| format!("{}",n))
  .collect::<Vec<String>>();

  data_frame!(key=h0, value=h2) 

  // let a = dst.get(1);
  // println!("{a:?}");

  // let a = dst.get_row(1);
  // println!("{a:?}");

  // let a = dst.get_columns();
  // let b = &a[0].iter();

}



#[cfg(feature="rpixel")]
extendr_module! {
  mod pixel;
  impl PixelI32;
  impl RpxLog;
  fn value_with_unit;
  fn polars_test;
}


#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn it_works() {
    let a = PixelI32::new(4,4);
    let b = a.get_index("bayer");
    println!("{:?}",b);
  }

  #[test]
  fn it_works2() {
    // polars_test();
  }
}