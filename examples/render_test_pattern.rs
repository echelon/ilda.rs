// Copyright (c) 2016 Brandon Thomas <bt@brand.io>
extern crate ilda;
extern crate image;

//use ilda::reader::read_file;
use ilda::parser::read_file;
//use image;
use std::fs::File;
use std::path::Path;

pub fn main() {
  let result = read_file("./examples/ildatest.ild")
    .ok().unwrap();
  println!("{:?}", result);

  let mut imgbuf = image::ImageBuffer::new(800, 800);

  let ref mut fout = File::create(&Path::new("output.png"))
    .unwrap();

  let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}

