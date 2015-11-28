// Copyright (c) 2015 Brandon Thomas <bt@brand.io>

use idtf::Header;
use idtf::IndexedPoint;
use idtf::TrueColorPoint;
use std::fs::File;
use std::io::Read;

// TODO: Revise errors.
pub enum Error {
  FileReadError,
  InvalidFile{reason: String},
  InvalidFormat,
}

/// Read ILDA data from a file.
pub fn read_file(filename: &str) -> Result<Vec<Header>, Error> {
  let mut contents = Vec::new();

  match File::open(filename) {
    Err(_) => { 
      return Err(Error::FileReadError); 
    },
    Ok(mut file) => {
      match file.read_to_end(&mut contents) {
        Err(_) => { 
          return Err(Error::FileReadError); 
        },
        Ok(_) => {},
      }
    }
  }

  read_bytes(&contents[..])
}

const HEADER_SIZE : usize = 32;
const INDEXED_3D_DATA_SIZE: usize = 8;
const INDEXED_2D_DATA_SIZE: usize = 8;
const ILDA_HEADER : [u8; 4] = [73u8, 76u8, 68u8, 65u8]; // "ILDA" in ASCII

/// Read ILDA data from raw bytes.
pub fn read_bytes(ilda_bytes: &[u8]) -> Result<Vec<Header>, Error> {
  if ilda_bytes.len() < 32 {
    println!("Error C");
    return Err(Error::InvalidFile { reason: "File too short.".to_string() });
  }

  let mut vec = Vec::new();
  let mut i : usize = 0; 

  match read_header(&ilda_bytes[i .. i + HEADER_SIZE]) {
    Err(err) => { 
      return Err(err); 
    },
    Ok(mut header) => {
      read_data(&mut header, &ilda_bytes[i + HEADER_SIZE ..]);
      vec.push(header);
    },
  }

  Ok(vec)
}

fn read_header(header_bytes: &[u8]) -> Result<Header, Error> {
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

fn read_data(header: &mut Header, bytes: &[u8] ) -> Result<Header, Error> {
  match header {
    &mut Header::IndexedFrame { records, is_3d, ref mut points, .. } => {
      if (is_3d) {
        let until = records as usize * INDEXED_3D_DATA_SIZE;
        let mut i = 0;

        while i < until {
          let data_bytes = &bytes[i .. i + INDEXED_3D_DATA_SIZE];
          let x           = read_u16(&data_bytes[0..2]) as i16; // TODO: i16
          let y           = read_u16(&data_bytes[2..4]) as i16; // TODO: i16
          let z           = read_u16(&data_bytes[4..6]) as i16; // TODO: i16
          let status      = data_bytes[6]; // TODO: Bitmask
          let color_index = data_bytes[7];

          let point = IndexedPoint {
            x: x,
            y: y,
            z: z,
            is_last_point: false,
            is_blank: false,
            color_index: color_index,
          };

          points.push(point);

          i += INDEXED_3D_DATA_SIZE;
        }
      } else {
        // TODO: Cleanup.
      }

      return Err(Error::FileReadError); // TODO: Return type ?
    },
    &mut Header::TrueColorFrame { .. } => {
      return Err(Error::FileReadError);
    },
    &mut Header::ColorPalette { .. } => {
      return Err(Error::FileReadError);
    },
  }

  Err(Error::FileReadError)
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

