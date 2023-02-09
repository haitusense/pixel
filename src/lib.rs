pub fn type_of<T>(_: T) -> String {
  let a = std::any::type_name::<T>();
  return a.to_string();
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pixel<T: Clone + Copy + Default + 'static>{
  width: usize,
  height: usize,
  data: Vec<T>,
}

impl<T> std::ops::Index<usize> for Pixel<T> where T: Clone + Copy + Default {
  type Output = T;
  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl<T> std::ops::IndexMut<usize> for Pixel<T> where T: Clone + Copy + Default {
  fn index_mut(&mut self, index: usize) -> &mut T {
    &mut self.data[index]
  }
}

impl <T> Pixel<T> where T: Clone + Copy + Default {

  pub fn create(width:usize, height:usize) -> Self{
    Self {
      width: width,
      height: height,
      data: vec![Default::default(); width*height]
    }
  }

  pub fn create_from_array(width:usize, height:usize, v:&[T]) -> Self{
    Self {
      width: width,
      height: height,
      data: v.to_vec()
    }
  }
 
  pub fn width(&self) -> usize { self.width }
  
  pub fn height(&self) -> usize { self.height }

  pub fn size(&self) -> usize { self.width * self.height }

  pub fn pixel(&self, x:i32, y:i32) -> Option<T> {
    let w = self.width as i32;
    let h = self.height as i32;
    if x < 0 || w <= x  { return None; }
    if y < 0 || h <= y  { return None; }

    let index = (x + y * self.width as i32) as usize;
    let dst = self.data[index];
    Some(dst)
  }

  pub fn set_pixel(&mut self, x:i32, y:i32, val:T) -> Option<()>{
    let w = self.width as i32;
    let h = self.height as i32;
    if x < 0 || w <= x  { return None; }
    if y < 0 || h <= y  { return None; }

    let index = (x + y * self.width as i32) as usize;
    self.data[index] = val;
    Some(())
  }

  pub fn get_array(&self) -> &[T] {
    &self.data
  }

}


