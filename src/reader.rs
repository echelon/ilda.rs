// Copyright (c) 2015 Brandon Thomas <bt@brand.io>

use idtf::*;
use std::fs::File;
use std::io::Read;

pub enum Error {
  FileReadError,
}

pub fn read_file(filename: &str) -> Result<Vec<Header>, Error> {
  let mut contents = String::new();

  match File::open(filename) {
    Err(_) => { return Err(Error::FileReadError); },
    Ok(mut file) => {
      // TODO: Not reading a string! Reading binary!
      match file.read_to_string(&mut contents) {
        Err(_) => { return Err(Error::FileReadError); },
        Ok(_) => {},
      }
    }
  }

  Err(Error::FileReadError)
}

