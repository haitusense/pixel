use extendr_api::prelude::*;


#[cfg(feature="rpixel", extendr)]
pub fn rpixel_hello() -> &'static str {
  "Hello rpixel!"
}

extendr_module! {
  mod pixel;
  fn rpixel_hello;
}
