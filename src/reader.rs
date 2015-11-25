// Copyright (c) 2015 Brandon Thomas <bt@brand.io>

use idtf::*;
use std::fs::File;
use std::io::Read;

pub enum Error {
  FileReadError,
  InvalidFile{reason: String},
  InvalidFormat,
}

pub fn read_file(filename: &str) -> Result<Vec<Header>, Error> {
  let mut contents = Vec::new();

  match File::open(filename) {
    Err(_) => { 
      println!("Error A");
      return Err(Error::FileReadError); 
    },
    Ok(mut file) => {
      // TODO: Not reading a string! Reading binary!
      match file.read_to_end(&mut contents) {
        Err(_) => { 
          println!("Error B");
          return Err(Error::FileReadError); 
        },
        Ok(_) => {},
      }
    }
  }

  if contents.len() < 32 {
    println!("Error C");
    return Err(Error::InvalidFile { reason: "File too short.".to_string() });
  }

  let header_slice : &[u8] = &contents[0..32];

  println!("Byte: {}", header_slice[0] );
  println!("Byte: {}", header_slice[1] );
  println!("Byte: {}", header_slice[2] );
  println!("Byte: {}", header_slice[3] );

  assert_eq!(&contents[0..4], &ILDA_HEADER);


  Err(Error::FileReadError)
}

const ILDA_HEADER : [u8; 4] = [73u8, 76u8, 68u8, 65u8]; // "ILDA"


pub fn parse_header(bytes: [u8; 32]) -> Result<Header, Error> {
  if &bytes[0..4] != &ILDA_HEADER {
    return Err(Error::InvalidFormat);
  }

  // Read "format code" byte.
  let header = match bytes[7] {
    0u8 => { 
      Header::TrueColorFrame {
        frame_name: parse_name(&bytes[8..17]),
        company_name: parse_name(&bytes[16..25]),
        projector_number: 0,
        frame_number: 0,
        total_frames: 0,
        is_3d: false,
        points: Vec::new(),
      }
    },
    1u8 => { 
      Header::TrueColorFrame {
        frame_name: None,
        company_name: None,
        projector_number: 0,
        frame_number: 0,
        total_frames: 0,
        is_3d: true,
        points: Vec::new(),
      }
    },
    _ => {
      return Err(Error::InvalidFormat);
    }
  };

  Ok(header)
}

fn parse_name(bytes: &[u8]) -> Option<String> {
  None
}

