use anyhow::Result;
use anyhow::Context as _;

use regex::Regex;

pub fn unit_to_f64(unit:&str) -> Result<f64> {
  Ok(match unit.trim() {    
    "GV" => 1e9,
    "MV" => 1e6,
    "kV" => 1e3,
    "V" => 1f64,
    "mV" => 1e-3,
    "uV" => 1e-6,
    "nV" => 1e-9,
    "pV" => 1e-12,
    "fV" => 1e-15,
    
    "GA" => 1e9,
    "MA" => 1e6,
    "kA" => 1e3,
    "A" => 1f64,
    "mA" => 1e-3,
    "uA" => 1e-6,
    "nA" => 1e-9,
    "pA" => 1e-12,

    "G" => 1e9,
    "M" => 1e6,
    "k" => 1e3,
    "m" => 1e-3,
    "u" => 1e-6,
    "n" => 1e-9,
    "p" => 1e-12,
    "" => 1f64,
    _ => 0f64
  })
}

pub fn value_with_unit_to_f64(src:&str) -> Result<f64> {
  let re = Regex::new(r"([\-.0-9]+)([ a-zA-Z]*)")?;
  let dst = re.captures(src).context("caps err")?;

  let val : f64 = dst.get(1).unwrap().as_str().parse()?;
  let unit = dst.get(2).unwrap().as_str();

  Ok(val * unit_to_f64(unit)?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() -> Result<()> {
    let src = "-2.83mA";
    let dst = value_with_unit_to_f64(src)?;
    println!("{dst:?}");
    Ok(())
  }

}