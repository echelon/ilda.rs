// Copyright (c) 2015 Brandon Thomas <bt@brand.io>
extern crate ilda;

use ilda::reader::read_file;


pub fn main() {
  let result = read_file("./examples/ildatest.ild").ok().unwrap();
  println!("{:?}", result);


  let vec = [0, 1, 2, 3, 4, 5, 6];

  println!("{}", (&vec[0..4]).len());
}

