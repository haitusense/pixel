#[cfg(not(feature="rpixel"))]
use pixel_proc::dummy as extendr;
#[cfg(feature="rpixel")]
use extendr_api::prelude::*;

#[extendr]
pub fn rpixel_hello() -> &'static str {
  "Hello rpixel!"
}

#[cfg(feature="rpixel")]
extendr_module! {
  mod pixel;
  fn rpixel_hello;
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