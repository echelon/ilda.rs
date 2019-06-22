//! Low level writing of ILDA frames.

use data::IldaEntry;
use data::ILDA_HEADER;
use error::IldaError;
use std::fs::File;
use std::io::{BufWriter, Write};

/// A struct that can be used to write IldaEntries to an underlying Write object
pub struct IldaWriter<W: Write> where W: Write {
  inner: W,
}

impl IldaWriter<BufWriter<File>> {
  /// Crate an IldaWriter that writes to a file. (Buffered)
  pub fn create(filename: &str) -> Result<IldaWriter<BufWriter<File>>, IldaError> {
    let inner = BufWriter::new(File::create(filename)?);
    Ok(IldaWriter::new(inner))
  }
}

fn u16_be(value: u16) -> [u8; 2] {
    [(value >> 8) as u8, (value & 0xFF) as u8]
}

fn str_8c(value: Option<String>) -> [u8; 8] {
  let value = value.as_ref().map_or("", String::as_ref);
  let mut arr = [0; 8];
  let bytes = value.as_bytes();

  for i in 0..bytes.len().min(8) {
    arr[i] = bytes[i]
  }

  arr
}

impl<W: Write> IldaWriter<W> where W: Write {
  /// Create a new IldaWriter. If writing to a file use `create` instead.
  /// Using a buffered writer is highly recommended.
  pub fn new(inner: W) -> IldaWriter<W> {
    IldaWriter { inner }
  }

  /// Write an IldaEntry to the undelying writer.
  pub fn write(&mut self, entry: IldaEntry) -> Result<(), IldaError> {
    match entry {
      IldaEntry::HeaderEntry(header) => {
        self.inner.write(&ILDA_HEADER)?;
        self.inner.write(&[0, 0, 0, header.format_code])?;
        self.inner.write(&str_8c(header.name))?;
        self.inner.write(&str_8c(header.company_name))?;
        self.inner.write(&u16_be(header.record_count))?;
        self.inner.write(&u16_be(header.number))?;
        self.inner.write(&u16_be(header.total_frames))?;
        self.inner.write(&[header.projector_number])?;
        self.inner.write(&[0])?;
      }
      IldaEntry::TcPoint3dEntry(point) => {
        self.inner.write(&u16_be(point.x as u16))?;
        self.inner.write(&u16_be(point.y as u16))?;
        self.inner.write(&u16_be(point.z as u16))?;
        self.inner.write(&[point.status_code])?;
        self.inner.write(&[point.b])?;
        self.inner.write(&[point.g])?;
        self.inner.write(&[point.r])?;
      }
      IldaEntry::TcPoint2dEntry(point) => {
        self.inner.write(&u16_be(point.x as u16))?;
        self.inner.write(&u16_be(point.y as u16))?;
        self.inner.write(&[point.status_code])?;
        self.inner.write(&[point.b])?;
        self.inner.write(&[point.g])?;
        self.inner.write(&[point.r])?;
      }
      IldaEntry::ColorPaletteEntry(palette) => {
        self.inner.write(&[palette.r])?;
        self.inner.write(&[palette.g])?;
        self.inner.write(&[palette.b])?;
      }
      IldaEntry::IdxPoint3dEntry(point) => {
        self.inner.write(&u16_be(point.x as u16))?;
        self.inner.write(&u16_be(point.y as u16))?;
        self.inner.write(&u16_be(point.z as u16))?;
        self.inner.write(&[point.status_code])?;
        self.inner.write(&[point.color_index])?;
      }
      IldaEntry::IdxPoint2dEntry(point) => {
        self.inner.write(&u16_be(point.x as u16))?;
        self.inner.write(&u16_be(point.y as u16))?;
        self.inner.write(&[point.status_code])?;
        self.inner.write(&[point.color_index])?;
      }
    };

    Ok(())
  }
}
