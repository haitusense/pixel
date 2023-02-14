use std::collections::HashMap;

use anyhow::{Result, bail};
use anyhow::Context as _;
use serde::Deserialize;
use serde_yaml::Value;

use super::bit_op::{*};

#[derive(Default, Debug, Copy, Clone)]
pub struct SyncCode{
  pub sof:u64,
  pub sol:u64,
  pub eof:u64,
  pub eol:u64,
  pub sync_mask:u64,
  pub pixel_depth:usize,
  pub pixel_mask:u64,
	pub trainingcode:u64,
	pub width:u32,
	pub height:u32,
	pub frame:usize,
}

#[derive(Debug, Copy, Clone)]
pub enum SyncType {
	SOF(SyncCnt),
	SOL(SyncCnt),
	EOF(SyncCnt),
	EOL(SyncCnt)
}

impl SyncCode {
  pub fn setdepth(&mut self, val: u32) {
		self.pixel_depth = val as usize;
    self.pixel_mask = 2u64.pow(val) - 1u64;
  }

	pub fn checkcode(&self, code: u64) -> (bool, bool, bool, bool) {
		let a = code & self.sync_mask == self.sof;
		let b = code & self.sync_mask == self.sol;
		let c = code & self.sync_mask == self.eof;
		let d = code & self.sync_mask == self.eol;
		(a, b ,c, d)
	}

	pub fn print(&self) {
    println!("SOF     {:0>1$b}", self.sof, 64);
		println!("SOL     {:0>1$b}", self.sol, 64);
		println!("EOF     {:0>1$b}", self.eof, 64);
		println!("EOL     {:0>1$b}", self.eol, 64);
		println!("mask    {:0>1$b}", self.sync_mask, 64);
		println!("TC      {:0>1$b}", self.trainingcode, 64);
		println!("TC mask {:0>1$b}", self.pixel_mask, 64);
	}
}

#[derive(Default, Debug, Copy, Clone)]
pub struct SyncCnt{
	sof: usize,
	sol: usize,
	eof: usize,
	eol: usize
}


pub fn create_index(src:&[u8], sync: SyncCode, _stride:usize, _depth:usize) -> Result<HashMap::<usize, SyncType>> {
	let mut dst = HashMap::<usize, SyncType>::new();
	let mut c = SyncCnt::default();
  for index in 0..src.len() * 8 {
    let (_, (_, code)) = match clip_bit_to_u64(&src, index){
      Ok(n) => n,
      Err(_) => bail!("cannot convert bainary : index = {}", index)
    };

		match sync.checkcode(code) {
			/*sof*/ (true, false, false, false) =>{
				c.sof += 1;
				c.sol = 0;
				c.eol = 0;
				dst.insert(index, SyncType::SOF(c));
			},
			/*sol*/ (false, true, false, false) =>{
				c.sol += 1;
				dst.insert(index, SyncType::SOL(c));
			},
			/*eof*/ (false, false, true, false) =>{
				c.eof += 1;
				dst.insert(index, SyncType::EOF(c));
				// return Ok(dst);
			}
			/*eol*/ (false, false, false, true) =>{
				c.eol += 1;
				dst.insert(index, SyncType::EOL(c));
			},
			/*other*/ (_, _, _, _) =>{ continue; },
		}
  }
	Ok(dst)
}


pub fn sync_to_pix(src:&[u8], sync: SyncCode, skipframe:usize) -> Result<Vec::<i32>> {
	let width = sync.width as usize;
	let height = sync.height as usize;
	
	let mut dst = vec![0i32;width * height];

	let (mut x_count, mut line_index, mut sof_count) = (0usize, 0usize, (skipframe + 1) as i32);

  for index in 0..src.len() * 8 {

    let (_, (_, code)) = match clip_bit_to_u64(&src, index){
      Ok(n) => n,
      Err(_) => bail!("cannot convert bainary : index = {}", index)
    };

		match (sof_count, sync.checkcode(code)) {
			/*sof*/ (_, (true, false, false, false)) =>{
				if sof_count == 0 { return Ok(dst); };
				x_count = 0;
				line_index = 0;
				sof_count -= 1;
			},
			/*sol*/ (0, (false, true, false, false)) =>{
				if x_count > width * 14 { line_index += 1; }
				x_count = 0;
			},
			/*eof*/ (0, (false, false, true, false)) =>{
				return Ok(dst);
			}
			/*eol*/ (0, (false, false, false, true)) =>{
				line_index += 1;
				x_count = 0;
			},
			/*pixel*/ (0, (false, false, false, false)) =>{
				x_count += 1;
				let x = x_count / sync.pixel_depth; 
				if line_index >= height { return Ok(dst); };
				if x_count % sync.pixel_depth == 0 && (x-1) < width {
					dst[x-1 + line_index * width] = (code & sync.pixel_mask) as i32;
				}
			},
			/*other*/ (_, (_, _, _, _)) =>{ continue; },
		}
  }
  Ok(dst)
}


pub fn yml_to_sync(src:&str) -> Result<SyncCode> {
	let de = serde_yaml::Deserializer::from_str(src);
	let value = Value::deserialize(de).context("Failed to open yaml file")?;

  let create_sync = |sync:&str| -> Result<u64> {
		let dst = match value[sync].is_u64() {
			true => {
				// println!("is u64");
				value[sync].as_u64().context("is not binary num")?
			},
			false => {
				let _s = value[sync].as_str()
					.context("is not str")?
					.replace("0b", "").replace("_", "");
    		u64::from_str_radix(_s.as_str(), 2).context("is not binary num")?
			}
		};

    // println!("{}  {:0>2$b}", sync, dst, 64);
    Ok(dst)
  };

  let sof = create_sync("sof").context("key 'sof' not found in yaml")?;
  let sol = create_sync("sol").context("key 'sol' not found in yaml")?;
  let eof = create_sync("eof").context("key 'eof' not found in yaml")?;
  let eol = create_sync("eol").context("key 'eol' not found in yaml")?;
	let width = value["width"].as_u64().context("key 'width' not found in yaml")? as u32;
  let height = value["height"].as_u64().context("key 'height' not found in yaml")? as u32;
  // let sync_mask = 2u64.pow(value["mask"].as_u64().context("key 'mask' not found in yaml")? as u32) - 1;
	let sync_mask = create_sync("mask").context("key 'mask' not found in yaml")?;

  let trainingcode = create_sync("trainingcode").context("key 'trainingcode' not found in yaml")?;
  let pixel_depth = value["depth"].as_u64().context("key 'depth' not found in yaml")? as usize;
	let pixel_mask = 2u64.pow(pixel_depth as u32) - 1u64;

	let frame = match value["frame"].as_u64() {
		Some(n) => n as usize,
		None => 0
	};

	Ok(SyncCode { sof, sol, eof, eol, sync_mask, pixel_depth, pixel_mask, trainingcode, width, height, frame})
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
	fn it_works() -> Result<()>{
    let src = "
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

    let hoge = yml_to_sync(src)?;
		hoge.print();
    println!("{:?}", hoge);

    Ok(())
	}


}
