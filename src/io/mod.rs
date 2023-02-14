use anyhow::Result;
use anyhow::Context as _;

use std::fs;
use std::fs::File;
use std::io::{Read, Write, BufWriter};

use std::path::{Path, PathBuf};
use std::ffi::OsString;

mod bit_op;
mod lvds;

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

pub fn syncfile_to_i32(path:PathBuf, sync: &str, skipframe:usize) -> Result<Vec<i32>> {
  let mut src = Vec::new();
  let mut file = File::open(path)?;
  let _ = file.read_to_end(&mut src).context("read file err")?;
  let sync = lvds::yml_to_sync(sync)?;
  lvds::sync_to_pix(&src[..], sync, skipframe)
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

  #[test]
	fn it_works2() -> Result<()>{
    let sync = "
    sof : 0b_11111111111111_00000000000000_00000000000000_10101011000000
    sol : 0b_11111111111111_00000000000000_00000000000000_10000000000000
    eof : 0b_11111111111111_00000000000000_00000000000000_10110110000000
    eol : 0b_11111111111111_00000000000000_00000000000000_10011101000000
    mask : 0b11111111111111111111111111111111111111111111111111111111
    trainingcode : 0b_00011100001111
    depth : 14
    width : 334
    height : 2072
    ";
    let path = PathBuf::from("../50a/Ch0.bin");
		let a = syncfile_to_i32(path, sync, 2).unwrap();
    println!("{:?}", a[7]);
		println!("{:?}", a[8]);
		println!("{:?}", a[9]);
		println!("{:?}", a[10]);

    Ok(())
	}

}