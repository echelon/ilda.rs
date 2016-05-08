// Copyright (c) 2015-2016 Brandon Thomas <bt@brand.io>

use data::COLOR_PALETTE_SIZE;
use data::ColorPalette;
use data::Format;
use data::HEADER_SIZE;
use data::INDEXED_2D_DATA_SIZE;
use data::INDEXED_3D_DATA_SIZE;
use data::IldaEntry;
use data::IndexedPoint2d;
use data::IndexedPoint3d;
use data::RawHeader;
use data::TRUE_COLOR_2D_DATA_SIZE;
use data::TRUE_COLOR_3D_DATA_SIZE;
use data::TrueColorPoint2d;
use data::TrueColorPoint3d;
use std::fs::File;
use std::io::Read;

// "ILDA" in ASCII.
const ILDA_HEADER : [u8; 4] = [73u8, 76u8, 68u8, 65u8];

// TODO: Revise errors.
pub enum Error {
  FileReadError,
  InvalidFile{reason: String},
  InvalidFormat,
  Unimplemented, // TODO TEMP
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

  enum NextRead { Header, I3d, I2d, Color, Tc3d, Tc2d };

  let mut vec = Vec::new();
  let mut i : usize = 0;
  let mut next_read = NextRead::Header;
  let mut frames_to_read = 0;

  // TODO(echelon): This isn't very concise. 
  while i < ilda_bytes.len() {
    match next_read {
      NextRead::Header => {
        match read_header(&ilda_bytes[i .. i + HEADER_SIZE]) {
          Err(err) => {
            return Err(err);
          },
          Ok(mut header) => {
            next_read = match header.get_format() {
              Format::Indexed3d => NextRead::I3d,
              Format::Indexed2d => NextRead::I2d,
              Format::ColorPalette => NextRead::Color,
              Format::TrueColor3d => NextRead::Tc3d,
              Format::TrueColor2d => NextRead::Tc2d,
              Format::Unknown => {
                return Err(Error::InvalidFile { reason: "Bad format.".to_string() });
              },
            };

            frames_to_read = header.record_count;
            vec.push(IldaEntry::HeaderEntry(header));
            i += HEADER_SIZE;
          },
        };
      },
      NextRead::I3d => {
        let end = INDEXED_3D_DATA_SIZE * frames_to_read as usize;
        match IndexedPoint3d::read_bytes(&ilda_bytes[i .. i + end]) {
          Err(err) => {
            return Err(Error::InvalidFormat); // TODO: Better error
          },
          Ok(mut points) => {
            let mut entries = points.iter()
              .map(|x| IldaEntry::IdxPoint3dEntry(x.clone()))
              .collect();
            vec.append(&mut entries);
          },
        }
        next_read = NextRead::Header;
        i += end;
      },
      NextRead::I2d => {
        let end = INDEXED_2D_DATA_SIZE * frames_to_read as usize;
        match IndexedPoint2d::read_bytes(&ilda_bytes[i .. i + end]) {
          Err(err) => {
            return Err(Error::InvalidFormat); // TODO: Better error
          },
          Ok(mut points) => {
            let mut entries = points.iter()
              .map(|x| IldaEntry::IdxPoint2dEntry(x.clone()))
              .collect();
            vec.append(&mut entries);
          },
        }
        next_read = NextRead::Header;
        i += end;
      },
      NextRead::Color => {
        let end = COLOR_PALETTE_SIZE * frames_to_read as usize;
        match ColorPalette::read_bytes(&ilda_bytes[i .. i + end]) {
          Err(err) => {
            return Err(Error::InvalidFormat); // TODO: Better error
          },
          Ok(mut points) => {
            let mut entries = points.iter()
              .map(|x| IldaEntry::ColorPaletteEntry(x.clone()))
              .collect();
            vec.append(&mut entries);
          },
        }
        next_read = NextRead::Header;
        i += end;
      },
      NextRead::Tc3d => {
        let end = TRUE_COLOR_3D_DATA_SIZE * frames_to_read as usize;
        match TrueColorPoint3d::read_bytes(&ilda_bytes[i .. i + end]) {
          Err(err) => {
            return Err(Error::InvalidFormat); // TODO: Better error
          },
          Ok(mut points) => {
            let mut entries = points.iter()
              .map(|x| IldaEntry::TcPoint3dEntry(x.clone()))
              .collect();
            vec.append(&mut entries);
          },
        }
        next_read = NextRead::Header;
        i += end;
      },
      NextRead::Tc2d => {
        let end = TRUE_COLOR_2D_DATA_SIZE * frames_to_read as usize;
        match TrueColorPoint2d::read_bytes(&ilda_bytes[i .. i + end]) {
          Err(err) => {
            return Err(Error::InvalidFormat); // TODO: Better error
          },
          Ok(mut points) => {
            let mut entries = points.iter()
              .map(|x| IldaEntry::TcPoint2dEntry(x.clone()))
              .collect();
            vec.append(&mut entries);
          },
        }
        next_read = NextRead::Header;
        i += end;
      },
    };
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

