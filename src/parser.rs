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
use error::IldaError;
use std::fs::File;
use std::io::Read;

/// The ILDA format header; "ILDA" in ASCII.
const ILDA_HEADER : [u8; 4] = [73u8, 76u8, 68u8, 65u8];

pub fn read_file(filename: &str) -> Result<Vec<IldaEntry>, IldaError> {
  let mut contents = Vec::new();
  let mut file = File::open(filename)?;
  let _r = file.read_to_end(&mut contents);
  read_bytes(&contents[..])
}

/// Read ILDA data from raw bytes.
pub fn read_bytes(ilda_bytes: &[u8]) -> Result<Vec<IldaEntry>, IldaError> {
  if ilda_bytes.len() < 32 {
    return Err(IldaError::InvalidFile {
      reason: "File too short.".to_string()
    });
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
                return Err(IldaError::InvalidFile {
                  reason: "Bad format.".to_string()
                });
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
            return Err(IldaError::InvalidFormat); // TODO: Better error
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
            return Err(IldaError::InvalidFormat); // TODO: Better error
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
            return Err(IldaError::InvalidFormat); // TODO: Better error
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
            return Err(IldaError::InvalidFormat); // TODO: Better error
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
            return Err(IldaError::InvalidFormat); // TODO: Better error
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

fn read_header(header_bytes: &[u8]) -> Result<RawHeader, IldaError> {
  if header_bytes.len() != 32
      || &header_bytes[0..4] != &ILDA_HEADER {
    return Err(IldaError::InvalidFormat);
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

#[cfg(test)]
mod tests {
  use super::read_name;
  use super::read_i16;
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
  fn test_read_i16() {
    assert_eq!(read_i16(&[0u8, 0u8]), 0i16);
    assert_eq!(read_i16(&[0u8, 255u8]), 255i16);
    assert_eq!(read_i16(&[127u8, 255u8]), 32767i16);
    assert_eq!(read_i16(&[128u8, 0u8]), -32768i16);
    assert_eq!(read_i16(&[128u8, 255u8]), -32513i16);
    assert_eq!(read_i16(&[255u8, 0u8]), -256);
    assert_eq!(read_i16(&[255u8, 1u8]), -255);
    assert_eq!(read_i16(&[255u8, 255u8]), -1);
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

