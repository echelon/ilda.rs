// Copyright (c) 2015 Brandon Thomas <bt@brand.io>
extern crate ilda;

use ilda::reader::read_file;


pub fn main() {
  read_file("./examples/ildatest.ild");
}

