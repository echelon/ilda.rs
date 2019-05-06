// Copyright (c) 2015-2016 Brandon Thomas <bt@brand.io>

//! Low level parsing that returns headers and data fields closer to the
//! underlying ILDA data model.

use data::ILDA_HEADER;
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
use std::io::Cursor;
use std::io::Error;
use std::io::ErrorKind;

/// Read ILDA data from a file.
pub fn read_file(filename: &str) -> Result<Vec<IldaEntry>, IldaError> {
  let mut file = File::open(filename)?;
  stream_with_error(&mut file).collect()
}

/// Read ILDA data from raw bytes.
pub fn read_bytes(ilda_bytes: &[u8]) -> Result<Vec<IldaEntry>, IldaError> {
  let mut cursor = Cursor::new(ilda_bytes);
  stream_with_error(&mut cursor).collect()
}

/// Stream ILDA entries from a reader.
/// The iterator will panic if it encounters an error.
pub fn stream<'a>(reader: &'a mut Read) -> IldaEntryIterator<'a> {
  IldaEntryIterator(IldaEntryIteratorData::new(reader))
}

/// Stream ILDA entries (with error handling) from a reader.
pub fn stream_with_error<'a>(reader: &'a mut Read) -> IldaEntryIteratorWithError<'a> {
  IldaEntryIteratorWithError(IldaEntryIteratorData::new(reader))
}

/// Data for the Iterators.
struct IldaEntryIteratorData<'a> {
  source: &'a mut Read,
  current_format: Option<Format>,
  frames_to_read: u16
}

/// Iterator over IldaEntry items. Panics on error.
pub struct IldaEntryIterator<'a>(IldaEntryIteratorData<'a>);

/// Iterator over Result<IldaEntry, IldaError> items.
pub struct IldaEntryIteratorWithError<'a>(IldaEntryIteratorData<'a>);

impl<'a> Iterator for IldaEntryIterator<'a> {
  type Item = IldaEntry;

  fn next(&mut self) -> Option<Self::Item> {
    self.0._next().unwrap()
  }
}

impl<'a> Iterator for IldaEntryIteratorWithError<'a> {
  type Item = Result<IldaEntry, IldaError>;

  fn next(&mut self) -> Option<Self::Item> {
    return self.0._next().transpose()
  }
}

impl<'a> IldaEntryIteratorData<'a> {
  fn new(source: &'a mut Read) -> IldaEntryIteratorData<'a> {
    IldaEntryIteratorData {
      source,
      current_format: None,
      frames_to_read: 0
    }
  }

  fn _next(&mut self) -> Result<Option<IldaEntry>, IldaError> {
    if self.frames_to_read == 0 {
      // currentry no frames are expected to follow the stream, read new header
      let mut buffer = [0; HEADER_SIZE];

      // The following logic behaves like read_exact but return Ok(None) if it immediately encounters EOF
      let mut bytes_read = 0;
      while bytes_read < HEADER_SIZE {
        match self.source.read(&mut buffer[bytes_read..HEADER_SIZE]) {
          Ok(0) => return if bytes_read == 0 {
            Ok(None)
          }
          else {
            Err(IldaError::IoError { cause: Error::new(ErrorKind::UnexpectedEof, "unexpected end of header") })
          },
          Ok(size) => bytes_read += size,
          Err(cause) => return Err(IldaError::IoError { cause })
        }
      }

      let header = read_header( & buffer)?;

      self.frames_to_read = header.record_count;
      self.current_format = Some(header.get_format()?);
      return Ok(Some(IldaEntry::HeaderEntry(header)))
    }

    let entry = match self.current_format.as_ref().unwrap() {
      Format::Indexed3d => {
        let mut buffer = [0; INDEXED_3D_DATA_SIZE];
        self.source.read_exact( & mut buffer)?;
        let point = IndexedPoint3d::read_bytes( &buffer) ?.remove(0);
        IldaEntry::IdxPoint3dEntry(point)
      },
      Format::ColorPalette => {
        let mut buffer = [0; COLOR_PALETTE_SIZE];
        self.source.read_exact( & mut buffer)?;
        let point = ColorPalette::read_bytes( &buffer) ?.remove(0);
        IldaEntry::ColorPaletteEntry(point)
      },
      Format::Indexed2d => {
        let mut buffer = [0; INDEXED_2D_DATA_SIZE];
        self.source.read_exact( & mut buffer)?;
        let point = IndexedPoint2d::read_bytes( &buffer) ?.remove(0);
        IldaEntry::IdxPoint2dEntry(point)
      },
      Format::TrueColor3d => {
        let mut buffer = [0; TRUE_COLOR_3D_DATA_SIZE];
        self.source.read_exact( & mut buffer)?;
        let point = TrueColorPoint3d::read_bytes( &buffer) ?.remove(0);
        IldaEntry::TcPoint3dEntry(point)
      },
      Format::TrueColor2d => {
        let mut buffer = [0; TRUE_COLOR_2D_DATA_SIZE];
        self.source.read_exact( & mut buffer)?;
        let point = TrueColorPoint2d::read_bytes( &buffer) ?.remove(0);
        IldaEntry::TcPoint2dEntry(point)
      },
    };

    self.frames_to_read -= 1;

    Ok(Some(entry))
  }
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
