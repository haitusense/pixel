use anyhow::Result;
use anyhow::Context as _;

use std::fs;
use std::fs::File;
use std::io::{Read, Write, BufWriter};

use std::path::{Path, PathBuf};
use std::ffi::OsString;

mod bit_op;

/* read fn */

pub fn txtfile_to_u8(path:PathBuf) -> Result<Vec<u8>> {
  let src = fs::read_to_string(path)?;
  bit_op::txt_to_be_u8(&src)
}

pub fn binfile_to_u8(path:PathBuf) -> Result<Vec<u8>> {
  let mut src = Vec::new();
  let mut file = File::open(path)?;
  let _ = file.read_to_end(&mut src).context("read file err")?;
  Ok(src)
}

/* write fn */

pub fn i32_to_binfile(path:PathBuf, src:&[i32]) -> Result<()> {
  let mut buffer = BufWriter::new(File::create(path)?);
  for i in src.iter() {
    let buf: [u8;4] = i.to_be_bytes();
    buffer.write(&buf)?;
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works_path() -> Result<()> {

    Ok(())
  }

}