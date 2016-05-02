// Copyright (c) 2015-2016 Brandon Thomas <bt@brand.io>

use data::ColorPalette;
use data::IldaEntry;
use data::IndexedPoint2d;
use data::IndexedPoint3d;
use data::RawHeader;
use data::TrueColorPoint2d;
use data::TrueColorPoint3d;
use std::fs::File;
use std::io::Read;

// Various header and data payload sizes in bytes.
const HEADER_SIZE : usize = 32;
const COLOR_PALETTE_SIZE: usize = 3;
const INDEXED_2D_DATA_SIZE: usize = 6;
const INDEXED_3D_DATA_SIZE: usize = 8;
const TRUE_COLOR_2D_DATA_SIZE: usize = 8;
const TRUE_COLOR_3D_DATA_SIZE: usize = 10;

// "ILDA" in ASCII.
const ILDA_HEADER : [u8; 4] = [73u8, 76u8, 68u8, 65u8];

// TODO: Revise errors.
pub enum Error {
  FileReadError,
  InvalidFile{reason: String},
  InvalidFormat,
}

pub fn read_file(filename: &str) -> Result<Vec<IldaEntry>, Error> {
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

/// Read ILDA data from raw bytes.
pub fn read_bytes(ilda_bytes: &[u8]) -> Result<Vec<IldaEntry>, Error> {
  if ilda_bytes.len() < 32 {
    println!("Error C");
    return Err(Error::InvalidFile { reason: "File too short.".to_string() });
  }

  let mut vec = Vec::new();
  let mut i : usize = 0;

  while i < ilda_bytes.len() {
    //
    //
    // TODO: Update this...
    //
    //
    match read_header(&ilda_bytes[i .. i + HEADER_SIZE]) {
      Err(err) => {
        return Err(err);
      },
      Ok(mut header) => {
        /*read_data(&mut header, &ilda_bytes[i + HEADER_SIZE ..]);

        i += HEADER_SIZE;

        match &header {
          &OldHeader::IndexedFrame { records, is_3d, .. } => {
            if is_3d {
              i += INDEXED_3D_DATA_SIZE * records as usize;
            } else {
              i += INDEXED_2D_DATA_SIZE * records as usize;
            }
          },
          &OldHeader::TrueColorFrame { records, is_3d, .. } => {
            if is_3d {
              i += TRUE_COLOR_3D_DATA_SIZE * records as usize;
            } else {
              i += TRUE_COLOR_2D_DATA_SIZE * records as usize;
            }
          },
          &OldHeader::ColorPalette { records, .. } => {
            i += COLOR_PALETTE_SIZE * records as usize;
          },
        }*/

        vec.push(IldaEntry::HeaderEntry(header));
      },
    }
  }

  Ok(vec)
}

fn read_header(header_bytes: &[u8]) -> Result<RawHeader, Error> {
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

  Ok(RawHeader {
    reserved: 0, // TODO: Read in.
    format_code: header_bytes[7],
    name: name,
    company_name: company_name,
    record_count: number_of_records,
    number: frame_number,
    total_frames: total_frames,
    projector_number: projector_number,
    reserved_2: 0, // TODO: Read in.
  })
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

// TODO/FIXME: Does Rust's casting use 2's complement? Do some maths.
fn read_i16(bytes: &[u8]) -> i16 {
  (((bytes[0] as u16) << 8) | (bytes[1] as u16)) as i16
}

fn read_u16(bytes: &[u8]) -> u16 {
  ((bytes[0] as u16) << 8) | (bytes[1] as u16)
}

