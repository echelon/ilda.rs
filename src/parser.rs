// Copyright (c) 2015-2016 Brandon Thomas <bt@brand.io>

//! Low level parsing that returns headers and data fields closer to the
//! underlying ILDA data model.

use data::COLOR_PALETTE_SIZE;
use data::ColorPalette;
use data::Format;
use data::HEADER_SIZE;
use data::Header;
use data::INDEXED_2D_DATA_SIZE;
use data::INDEXED_3D_DATA_SIZE;
use data::IldaEntry;
use data::IndexedPoint2d;
use data::IndexedPoint3d;
use data::TRUE_COLOR_2D_DATA_SIZE;
use data::TRUE_COLOR_3D_DATA_SIZE;
use data::TrueColorPoint2d;
use data::TrueColorPoint3d;
use error::IldaError;
use std::fs::File;
use std::io::Read;

/// The ILDA format header; "ILDA" in ASCII.
const ILDA_HEADER : [u8; 4] = [73u8, 76u8, 68u8, 65u8];

/// Read ILDA data from a file.
pub fn read_file(filename: &str) -> Result<Vec<IldaEntry>, IldaError> {
  let mut contents = Vec::new();
  let mut file = File::open(filename)?;
  let _r = file.read_to_end(&mut contents);
  read_bytes(&contents[..])
}

#[derive(Debug)]
enum NextRead { Header, I3d, I2d, Color, Tc3d, Tc2d, NsTc }

/// Read ILDA data from raw bytes.
pub fn read_bytes(ilda_bytes: &[u8]) -> Result<Vec<IldaEntry>, IldaError> {
  if ilda_bytes.len() < 32 {
    return Err(IldaError::FileTooSmall);
  }

  let mut vec = Vec::new();
  let mut i : usize = 0;
  let mut next_read = NextRead::Header;
  let mut frames_to_read = 0;

  // TODO(echelon): This isn't very concise.
  while i < ilda_bytes.len() {
    println!("\nNext read: {:?}", next_read);

    match next_read {
      NextRead::Header => {
        let header = read_header(&ilda_bytes[i .. i + HEADER_SIZE])
            .map_err(|_| IldaError::InvalidHeader)?;

        println!("Header read succesfully");

        next_read = match header.get_format() {
          Format::Indexed3d => NextRead::I3d,
          Format::Indexed2d => NextRead::I2d,
          Format::ColorPalette => NextRead::Color,
          Format::TrueColor3d => NextRead::Tc3d,
          Format::TrueColor2d => NextRead::Tc2d,
          Format::NonstandardTrueColor => NextRead::NsTc,
          Format::Unknown => {
            println!("UNKNOWN HEADER");
            return Err(IldaError::InvalidHeader)
          },
        };

        println!("Very Next read: {:?}", next_read);

        frames_to_read = header.record_count;

        println!("Frames to read: {:?}", frames_to_read);

        vec.push(IldaEntry::HeaderEntry(header));
        i += HEADER_SIZE;
      },
      NextRead::I3d => {
        let end = INDEXED_3D_DATA_SIZE * frames_to_read as usize;
        let points = IndexedPoint3d::read_bytes(&ilda_bytes[i .. i + end])?;
        let mut entries = points.iter()
          .map(|x| IldaEntry::IdxPoint3dEntry(x.clone()))
          .collect();
        vec.append(&mut entries);
        next_read = NextRead::Header;
        i += end;
      },
      NextRead::I2d => {
        let end = INDEXED_2D_DATA_SIZE * frames_to_read as usize;
        let points = IndexedPoint2d::read_bytes(&ilda_bytes[i .. i + end])?;
        let mut entries = points.iter()
          .map(|x| IldaEntry::IdxPoint2dEntry(x.clone()))
          .collect();
        vec.append(&mut entries);
        next_read = NextRead::Header;
        i += end;
      },
      NextRead::Color => {
        let end = COLOR_PALETTE_SIZE * frames_to_read as usize;
        let points = ColorPalette::read_bytes(&ilda_bytes[i .. i + end])?;
        let mut entries = points.iter()
          .map(|x| IldaEntry::ColorPaletteEntry(x.clone()))
          .collect();
        vec.append(&mut entries);
        next_read = NextRead::Header;
        i += end;
      },
      NextRead::Tc3d => {
        let end = TRUE_COLOR_3D_DATA_SIZE * frames_to_read as usize;
        let points = TrueColorPoint3d::read_bytes(&ilda_bytes[i .. i + end])?;
        let mut entries = points.iter()
          .map(|x| IldaEntry::TcPoint3dEntry(x.clone()))
          .collect();
        vec.append(&mut entries);
        next_read = NextRead::Header;
        i += end;
      },
      NextRead::Tc2d => {
        let end = TRUE_COLOR_2D_DATA_SIZE * frames_to_read as usize;
        let points = TrueColorPoint2d::read_bytes(&ilda_bytes[i .. i + end])?;
        let mut entries = points.iter()
          .map(|x| IldaEntry::TcPoint2dEntry(x.clone()))
          .collect();
        vec.append(&mut entries);
        next_read = NextRead::Header;
        i += end;
      },
      NextRead::NsTc => {
        // TODO.
      },
    };
  }

  Ok(vec)
}

fn read_header(header_bytes: &[u8]) -> Result<Header, IldaError> {
  if header_bytes.len() != 32 || &header_bytes[0..4] != &ILDA_HEADER {
    return Err(IldaError::InvalidHeader);
  }

  let name              = read_name(&header_bytes[8..16]);
  let company_name      = read_name(&header_bytes[16..24]);
  let number_of_records = read_u16(&header_bytes[24..26]);
  let frame_number      = read_u16(&header_bytes[26..28]);
  let total_frames      = read_u16(&header_bytes[28..30]);
  let projector_number  = header_bytes[31];

  println!("Name: {:?}", name);
  println!("Company Name: {:?}", company_name);
  println!("Format code: {:?}", header_bytes[7]);

  Ok(Header {
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

fn read_format3_header(header_bytes: &[u8]) -> Result<Header, IldaError> {
  if header_bytes.len() != 16 || &header_bytes[0..4] != &ILDA_HEADER {
    return Err(IldaError::InvalidHeader);
  }

  // TODO WIP
  return Err(IldaError::InvalidHeader);
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
  use super::read_name;
  use super::read_u16;

  #[test]
  fn test_read_name() {
    assert_eq!(read_name(&[0, 0, 0, 0]), None);
    assert_eq!(read_name(&[0, 100, 100, 100]), None);
    assert_eq!(read_name(&[102, 111, 111]), Some("foo".to_string()));
    assert_eq!(read_name(&[102, 111, 111, 0, 111]),
               Some("foo".to_string()));
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
