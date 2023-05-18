use std::env;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use std::fs::*;
use std::io::*;
use chrono::Local;
fn main() -> std::io::Result<()>{
  let args: Vec<String> = env::args().collect();
  let taskname = &args[1];

  let date = Local::today();
  let time = Local::now();
  let log_line = time.format("%H:%M:%S\t").to_string() + taskname;
  let filename = date.format("%Y%m%d").to_string() + ".lifelog";
  let mut f = File::create(filename)?;
  f.write(String::from(log_line).as_bytes())?;
  f.sync_data()?;
  Ok(())
}
/************
fn parse_to_filename(old_str: &str) -> &str {
  let replaced_words:&[char] = &[':', '*', '?', '/','+','-'];
  let trimed_str = old_str.trim_matches(replaced_words);
  trimed_str
}
************/
