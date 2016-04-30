// Copyright (c) 2015 Brandon Thomas <bt@brand.io>
extern crate ilda;

use ilda::parser::read_file;

pub fn main() {
  let result = read_file("./examples/files/ildatest.ild").ok().unwrap();
  println!("{:?}", result);
}

