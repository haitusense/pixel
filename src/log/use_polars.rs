use polars::prelude::*;


pub fn logheader_to_df(src:&String) -> Result<DataFrame> {
  let mut df = DataFrame::default();

  let re = Regex::new(RE_HEADERS.replace("\n", "").as_str()).unwrap();
  let dst = re.captures(src.as_str()).context("caps err")?;
  let body = dst.get(0).unwrap().as_str();

  // let re = Regex::new(RE_HEADER.replace("\n", "").as_str()).unwrap();
  // for caps in re.captures_iter(body) {
  //   let key = caps.get(1).unwrap().as_str();
  //   let val = caps.get(2).unwrap().as_str();
  //   let s = Series::new(key, vec![val]);
  //   println!("{caps:?}");
  //   df = df
  //     .lazy()
  //     .with_columns(vec![s.lit()])
  //     .collect()
  //     .unwrap();
  // }

  Ok(df)
}

pub fn logbody_to_df(src:&String) -> Result<DataFrame> {
  let mut df = DataFrame::default();

  let re = Regex::new(RE_BODY.replace("\n", "").as_str()).unwrap();
  for caps in re.captures_iter(src.as_str()) {
    let tcnt = caps.get(1).unwrap().as_str();
    let site = caps.get(2).unwrap().as_str();
    let body = caps.get(3).unwrap().as_str();
    let testtime = caps.get(4).unwrap().as_str();
    let t_unit = caps.get(5).unwrap().as_str();
    let result = caps.get(6).unwrap().as_str();
    // println!("c : {tcnt} s : {site} t : {testtime} {t_unit} result : {result}");
    // df = df
    //   .lazy()
    //   .with_columns( vec![
    //     lit(tcnt).alias("tcnt"),
    //     lit(site).alias("site"),
    //     lit(body).alias("body"),
    //     lit(testtime).alias("testtime"),
    //     lit(t_unit).alias("t_unit"),
    //     lit(result).alias("result")
    //   ])
    //   .collect()
    //   .unwrap();
  }

  Ok(df)
}

pub trait Rpx {
  fn rpx_write(&mut self, path:&str, kind:&str);
}

impl Rpx for DataFrame {
  fn rpx_write(&mut self, path:&str, kind:&str) {
    match kind{
      "csv" => {
        let mut file = std::fs::File::create(path).unwrap();
        CsvWriter::new(&mut file).finish(self).unwrap();
      },
      "parquet" =>{
        let mut file = std::fs::File::create(path).unwrap();
        ParquetWriter::new(&mut file).finish(self).unwrap();
      },
      _ =>{
      }
    }
  }
}
// ParquetWriter gcc通らない


// pub fn to_csv(df: DataFrame) -> Result<String> {
//   // let mut buf = vec![];
//   // {
//   //   let mut f = BufWriter::new(&mut buf);
//   //   CsvWriter::new(&mut f).finish(&mut df).unwrap();
//   // }
//   // let dst = buf.iter().map(|&s| s as char).collect::<String>();
//   // println!("{}",dst);
  
//   let mut f = BufWriter::new(Vec::new());
//   CsvWriter::new(&mut f).finish(&mut df).unwrap();
//   let bytes = f.into_inner()?;
//   let dst = String::from_utf8(bytes)?;
//   // let dst = str::from_utf8(&bytes).unwrap();
//   Ok(dst)
// }

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn it_works1() -> Result<()> {
//     let src = read_logfile("../dummy.txt")?;

//     let df1 = logheader_to_df(&src)?;
//     println!("{df1:?}");

//     let df2 = logbody_to_df(&src)?;
//     println!("{df2:?}");

//     // let dst = df1.join(&df2, [], ["Name"], JoinType::Inner, None).unwrap();
   

//     Ok(())
//   }

//   #[test]
//   fn it_works2() -> Result<()> {
//     let src = read_logfile("../dummy.txt")?;
//     let mut df = logbody_to_df(&src)?;
//     println!("{df:?}");
    
//     df.rpx_write("temp.csv", "csv");
//     df.rpx_write("temp.parquet", "parquet");

//     Ok(())
//   }

//   #[test]
//   fn it_works_df() -> Result<()> {
//     let mut df = df!(
//       "A" => &["a", "b", "a"],
//       "B" => &[1, 3, 5],
//       "C" => &[10, 11, 12],
//       "D" => &[2, 4, 6]
//     )?;
//     println!("{df:?}");
//     Ok(())
//   }

//   #[test]
//   fn it_works_df2() -> Result<()> {
//     let mut df1: DataFrame = df!(
//       "Fruit" => &["Apple", "Banana", "Pear"],
//       "Origin" => &["America", "Hawai", "Italy"],
//       "Phosphorus (mg/100g)" => &[11, 22, 12]).unwrap();
//     let df2: DataFrame = df!(
//       "Name" => &["Apple", "Banana", "Pear"],
//       "Origin" => &["France", "Hawai", "Italy"],
//       "Potassium (mg/100g)" => &[107, 358, 115]).unwrap();
    
//     let dst = df1.join(&df2, ["Fruit"], ["Name"], JoinType::Inner, None).unwrap();
//     println!("{dst:?}");

//     // let dst2 = df1.join(&df2, ["Fruit"], ["Name"], JoinType::Left, None).unwrap();
//     // println!("{dst2:?}");


//     let _ = df1.extend(&df2).unwrap();
//     println!("{df1:?}");

//     Ok(())
//   }

//   #[test]
//   fn it_works_df3() -> Result<()> {
//     let df1: DataFrame = df!(
//       "F" => &["Apple"],
//       "O" => &["America"],
//     ).unwrap();
//     let df2: DataFrame = df!(
//       "Name" => &["Apple", "Banana", "Pear"],
//       "Origin" => &["France", "Hawai", "Italy"],
//       "Potassium (mg/100g)" => &[107, 358, 115]
//     ).unwrap();
    
//     // let a = df1.lazy();
//     // let b = df2.lazy();
//     // pl.concat([df_1,df_2]) 新しい単一のロケーションにコピー
//     // df_1.vstack(df_2) コピーなしでリンク的に接続（メモリ上は分離のまま）
//     // df_1.extend(df_2) df_2をコピーしてdf_1にappend

//     // let dst = df1.vstack(&df2).unwrap();
//     // let dst = concat([a, b], false, false)
//     //   .unwrap()
//     //   .collect()
//     //   .unwrap();

//     // let dst = polars::functions::diag_concat_df(&[df1,df2]).unwrap();
//     // println!("{dst:?}");

//     let dst = polars::functions::hor_concat_df(&[df1,df2]).unwrap();
//     println!("{dst:?}");

//     Ok(())
//   }

//   #[test]
//   // fn it_works_df4() -> Result<()> {
//   //   let df1: DataFrame = df!(
//   //     "A" => &["AA"],
//   //     "B" => &["BB"],
//   //   ).unwrap();
//   //   let mut df2: DataFrame = df!(
//   //     "C" => &[1, 2, 3],
//   //     "D" => &[4, 5, 6],
//   //     "E" => &[107, 358, 115]
//   //   ).unwrap();

//   //   let dst = df2.with_column_and_schema(df1.columns(names)).unwrap();


//   //   println!("{dst:?}");

//   //   Ok(())
//   // }


//   fn it_works_df4() -> Result<()> {
//     let df1: DataFrame = df!(
//       "A" => &["AA"],
//       "B" => &["BB"],
//     ).unwrap();
//     let mut df2: DataFrame = df!(
//       "C" => &[1, 2, 3],
//       "D" => &[4, 5, 6],
//       "E" => &[107, 358, 115]
//     ).unwrap();
//     // LazyFrame::sink_parquet(self, path, options)

//     // let df = LazyFrame::scan_parquet("../datasets/foods1.parquet", ScanArgsParquet::default())?
//     // .select([
//     //     all(),
//     //     cols(["fats_g", "sugars_g"]).sum().suffix("_summed"),
//     // ])
//     // .collect()?;
//     // println!("{df:?}");

//     Ok(())
//   }


// }