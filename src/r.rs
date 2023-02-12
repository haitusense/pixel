use extendr_api::prelude::*;

#[extendr]
pub fn rpixel_hello() -> &'static str {
  "Hello world!"
}

extendr_module! {
  mod pixel;
  fn rpixel_hello;
}
