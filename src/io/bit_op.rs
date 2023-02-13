use anyhow::{Result, bail};
// use anyhow::Context as _;

use nom::number::complete::{be_u128};
use nom::bytes::complete::{take};
use nom::IResult;

pub trait Flag {
  fn flag_up(&mut self, bit: usize);
  fn flag_down(&mut self, bit: usize);
  fn flag_up_rev(&mut self, bit: usize);
  fn flag_down_rev(&mut self, bit: usize);
  fn flag_clear(&mut self);
}


impl Flag for u8 {
  fn flag_up(&mut self, bit: usize) { *self |= 1u8 << bit; }

  fn flag_down(&mut self, bit: usize){ *self &= !(1u8 << bit); }
  
  fn flag_up_rev(&mut self, bit: usize) { *self |= 0b_1000_0000u8 >> bit; }

  fn flag_down_rev(&mut self, bit: usize){ *self &= !(0b_1000_0000u8 >> bit); }

  fn flag_clear(&mut self){ *self =0 }
}


pub fn txt_to_be_u8(src:&str) -> Result<Vec::<u8>> {
  
  let (mut buf, mut count, mut dst) = (0u8, 0usize, Vec::<u8>::new());
  
  for c in src.chars() {
    match c {
      '0' => { buf.flag_down_rev(count); },
      '1' => { buf.flag_up_rev(count); },
      _ => { continue; }
    }
    match count {
      0..=6 => { count += 1; }
      7 => { dst.push(buf); buf.flag_clear(); count = 0 },
      _ => bail!("counter err")
    }
  }
  // 端数処理
  match count {
    1..=7 => { dst.push(buf); }
    _ => {}
  }
  
  Ok(dst)
}


pub fn clip_bit_to_u64(input: &[u8], index: usize) -> IResult<&[u8], (u128, u64)> {
  let src = match index {
    0..=120 => { // 0 <= index < 128
      let (_, buf) = be_u128(input)?;
      buf >> ((127 - index) / 8) * 8
    },
    _ => {
      let (input, _) = take(index / 8 - 15)(input)?;
      let (_, buf) = be_u128(input)?;
      buf
    },
  };
  let mut dst = src >> (7 - index % 8);
  dst &= 0x_0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF;
  Ok((input, (src, dst as u64)))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
	fn it_works1() -> Result<()> {
    let mut a :u8 = 1;
    for i in 0..8 {
      a.flag_up(i);
      println!("{:0>8b}",a);  
    }
    for i in 0..8 {
      a.flag_down(i);
      println!("{:0>8b}",a);  
    }
    println!("");
    for i in 0..8 {
      a.flag_up_rev(i);
      println!("{:0>8b}",a);  
    }
    for i in 0..8 {
      a.flag_down_rev(i);
      println!("{:0>8b}",a);  
    }
    Ok(())
  }

}
