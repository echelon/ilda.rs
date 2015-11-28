// Copyright (c) 2015 Brandon Thomas <bt@brand.io>

use idtf::*;
use std::fs::File;
use std::io::Read;

// TODO: Revise errors.
pub enum Error {
  FileReadError,
  InvalidFile{reason: String},
  InvalidFormat,
}

// TODO: Interface for reading passed &[u8].
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

  let result = read_header(&contents[0..32]);

  let mut vec = Vec::new();
  vec.push(result.ok().unwrap());
  Ok(vec)

  //Err(Error::FileReadError)
}

const ILDA_HEADER : [u8; 4] = [73u8, 76u8, 68u8, 65u8]; // "ILDA" in ASCII

pub fn read_header(header_bytes: &[u8]) -> Result<Header, Error> {
  if header_bytes.len() != 32 
      || &header_bytes[0..4] != &ILDA_HEADER {
    return Err(Error::InvalidFormat);
  }

  let name              = read_name(&header_bytes[8..16]);
  let company_name      = read_name(&header_bytes[16..24]);
  let number_of_records = read_u16(&header_bytes[24..26]);
  let frame_number      = read_u16(&header_bytes[26..28]);
  let total_frames      = read_u16(&header_bytes[28..30]);
  let projector_number  = header_bytes[31];

  // Read "format code" byte.
  let header = match header_bytes[7] {
    f @ 0u8 |
    f @ 1u8 => { 
      Header::IndexedFrame {
        frame_name: name,
        company_name: company_name,
        records: number_of_records,
        projector_number: projector_number,
        frame_number: frame_number,
        total_frames: total_frames,
        is_3d: f == 0u8,
        points: Vec::new(),
      }
    },
    2u8 => { 
      Header::ColorPalette {
        palette_name: name,
        company_name: company_name,
        records: number_of_records,
        projector_number: projector_number,
        palette_number: frame_number,
        colors: Vec:: new(),
      }
    },
    f @ 4u8 |
    f @ 5u8 => { 
      Header::TrueColorFrame {
        frame_name: name,
        company_name: company_name,
        records: number_of_records,
        projector_number: projector_number,
        frame_number: frame_number,
        total_frames: total_frames,
        is_3d: f == 4u8,
        points: Vec::new(),
      }
    },
    _ => {
      return Err(Error::InvalidFormat);
    }
  };

  Ok(header)
}

fn read_name(bytes: &[u8]) -> Option<String> {
  let mut name = String::with_capacity(8);
  for byte in bytes {
    if *byte == 0 {
      break;
    } else if *byte < 31 {
      continue; // unprintable characters
    } else {
      name.push(*byte as char);
    }
  }
  match name.len() {
    0 => None,
    _ => Some(name),
  }
}

fn read_u16(bytes: &[u8]) -> u16 {
  ((bytes[0] as u16) << 8) | (bytes[1] as u16)
}


#[cfg(test)]
mod tests {
  //use super::*;
  use super::read_name;
  use super::read_u16;

  #[test]
  fn test_read_name() {
    assert_eq!(read_name(&[0, 0, 0, 0]), None);
    assert_eq!(read_name(&[0, 100, 100, 100]), None);
    assert_eq!(read_name(&[102, 111, 111]), Some("foo".to_string()));
    assert_eq!(read_name(&[102, 111, 111, 0, 111]), Some("foo".to_string()));
  }

  #[test]
  fn test_read_u16() {
    assert_eq!(read_u16(&[0u8, 0u8]), 0u16);
    assert_eq!(read_u16(&[0u8, 100u8]), 100u16);
    assert_eq!(read_u16(&[0u8, 255u8]), 255u16);
    assert_eq!(read_u16(&[1u8, 0u8]), 256u16);
    assert_eq!(read_u16(&[255u8, 0u8]), 65280u16);
    assert_eq!(read_u16(&[255u8, 255u8]), 65535u16);
  }
}

