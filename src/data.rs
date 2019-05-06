// Copyright (c) 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Structures in the ILDA data model.

use error::IldaError;

/// Size of an ILDA header section in bytes.
pub const HEADER_SIZE : usize = 32;
/// Size of an ILDA color palette data section in bytes.
pub const COLOR_PALETTE_SIZE: usize = 3;
/// Size of an ILDA Indexed 2D point data section in bytes.
pub const INDEXED_2D_DATA_SIZE: usize = 6;
/// Size of an ILDA Indexed 3D point data section in bytes.
pub const INDEXED_3D_DATA_SIZE: usize = 8;
/// Size of an ILDA True Color 2D point data section in bytes.
pub const TRUE_COLOR_2D_DATA_SIZE: usize = 8;
/// Size of an ILDA True Color 3D point data section in bytes.
pub const TRUE_COLOR_3D_DATA_SIZE: usize = 10;
/// The ILDA format header; "ILDA" in ASCII.
pub const ILDA_HEADER: [u8; 4] = [73u8, 76u8, 68u8, 65u8];

/// The payload encoding formats currently supported.
#[allow(missing_docs)]
pub enum Format {
  ColorPalette,
  Indexed2d,
  Indexed3d,
  TrueColor2d,
  TrueColor3d,
}

/// A Raw ILDA header.
#[derive(Clone, Debug)]
pub struct Header {
  /// The first reserved portion of the ILDA header.
  pub reserved: u16,

  /// The format, or type, of the header.
  pub format_code: u8,

  /// The name of the frame or color palette.
  pub name: Option<String>, // TODO: Make this fixed-width.

  /// The name of the company.
  pub company_name: Option<String>, // TODO: Make this fixed-width.

  /// The number of records (eg. points) following this header.
  /// If 0, this is the EOF header.
  pub record_count: u16,

  /// If the frame is part of an animation, this is the frame number
  /// within the animation sequence. If this is a color palette header,
  /// it's the palette number.
  pub number: u16,

  /// The total number of frames within this sequence. If this is a
  /// color palette, this shall be `0`.
  pub total_frames: u16,

  /// The projector to display this frame on.
  pub projector_number: u8,

  /// The final reserved portion.
  pub reserved_2: u8,
}

impl Header {
  /// Returns the format of the header.
  pub fn get_format(&self) -> Result<Format, IldaError> {
    Ok(match self.format_code {
      0u8 => Format::Indexed3d,
      1u8 => Format::Indexed2d,
      2u8 => Format::ColorPalette,
      4u8 => Format::TrueColor3d,
      5u8 => Format::TrueColor2d,
      _ => return Err(IldaError::InvalidHeader)
    })
  }

  /// Create new Header
  pub fn new(
    format: Format,
    name: Option<String>,
    company_name: Option<String>,
    record_count: u16,
    number: u16,
    total_frames: u16,
    projector_number: u8
  ) -> Header {
    Header {
      reserved: 0,
      format_code: match format {
        Format::Indexed3d => 0u8,
        Format::Indexed2d => 1u8,
        Format::ColorPalette => 2u8,
        Format::TrueColor3d => 4u8,
        Format::TrueColor2d => 5u8
      },
      name,
      company_name,
      record_count,
      number,
      total_frames,
      projector_number,
      reserved_2: 0
    }
  }
}

fn build_status_code(is_blank: bool, is_last: bool) -> u8 {
  (if is_last { 1 << 7 } else { 0 } | if is_blank { 1 << 6 } else { 0 })
}

/// 3D Coordinates with Indexed Color (format 0)
#[derive(Clone, Debug, Default)]
pub struct IndexedPoint3d {
  /// X coordinate
  pub x: i16,
  /// Y coordinate
  pub y: i16,
  /// Z coordinate
  pub z: i16,
  /// Last point bit and blanking bit.
  pub status_code: u8,
  /// Index into color palette (if provided), or default color index.
  pub color_index: u8,
}

impl IndexedPoint3d {
  /// Read multiple `IndexedPoint3d` from raw bytes.
  pub fn read_bytes(bytes: &[u8])
      -> Result<Vec<IndexedPoint3d>, IldaError> {
    if bytes.len() % INDEXED_3D_DATA_SIZE != 0 {
      return Err(IldaError::InvalidData);
    }

    let size = bytes.len() / INDEXED_3D_DATA_SIZE;
    let mut out = Vec::with_capacity(size);

    for i in 0..size {
      let j = i * INDEXED_3D_DATA_SIZE;
      out.push(IndexedPoint3d {
        x: read_i16(&bytes[j .. j+2]),
        y: read_i16(&bytes[j+2 .. j+4]),
        z: read_i16(&bytes[j+4 .. j+6]),
        status_code: bytes[j+6],
        color_index: bytes[j+7],
      });
    }

    Ok(out)
  }

  /// Whether the point is a blanking point.
  pub fn is_blank(&self) -> bool {
    // 7th high order bit is the blanking bit.
    self.status_code & 64 == 64
  }

  /// Create a new IndexedPoint3d point.
  pub fn new(x: i16, y: i16, z: i16, color_index: u8, is_last: bool, is_blank: bool) -> IndexedPoint3d {
    IndexedPoint3d {
      x,
      y,
      z,
      status_code: build_status_code(is_blank, is_last),
      color_index
    }
  }
}

/// 2D Coordinates with Indexed Color (format 1)
#[derive(Clone, Debug, Default)]
pub struct IndexedPoint2d {
  /// X coordinate
  pub x: i16,
  /// Y coordinate
  pub y: i16,
  /// Last point bit and blanking bit.
  pub status_code: u8,
  /// Index into color palette (if provided), or default color index.
  pub color_index: u8,
}

impl IndexedPoint2d {
  /// Read multiple `IndexedPoint2d` from raw bytes.
  pub fn read_bytes(bytes: &[u8])
      -> Result<Vec<IndexedPoint2d>, IldaError> {
    if bytes.len() % INDEXED_2D_DATA_SIZE != 0 {
      return Err(IldaError::InvalidData);
    }

    let size = bytes.len() / INDEXED_2D_DATA_SIZE;
    let mut out = Vec::with_capacity(size);

    for i in 0..size {
      let j = i * INDEXED_2D_DATA_SIZE;
      out.push(IndexedPoint2d {
        x: read_i16(&bytes[j .. j+2]),
        y: read_i16(&bytes[j+2 .. j+4]),
        status_code: bytes[j+4],
        color_index: bytes[j+5],
      });
    }

    Ok(out)
  }

  /// Whether the point is a blanking point.
  pub fn is_blank(&self) -> bool {
    // 7th high order bit is the blanking bit.
    self.status_code & 64 == 64
  }

  /// Create a new IndexedPoint2d point.
  pub fn new(x: i16, y: i16, color_index: u8, is_last: bool, is_blank: bool) -> IndexedPoint2d {
    IndexedPoint2d {
      x,
      y,
      status_code: build_status_code(is_blank, is_last),
      color_index
    }
  }
}

/// Color Palette (format 2)
#[derive(Clone, Debug, PartialEq)]
pub struct ColorPalette {
  /// Red.
  pub r: u8,
  /// Green.
  pub g: u8,
  /// Blue.
  pub b: u8,
}

impl ColorPalette {
  /// Read multiple `ColorPalette` from raw bytes.
  pub fn read_bytes(bytes: &[u8]) -> Result<Vec<ColorPalette>, IldaError> {
    if bytes.len() % COLOR_PALETTE_SIZE != 0 {
      return Err(IldaError::InvalidData);
    }

    let size = bytes.len() / COLOR_PALETTE_SIZE;
    let mut out = Vec::with_capacity(size);

    for i in 0..size {
      let j = i * COLOR_PALETTE_SIZE;
      out.push(ColorPalette {
        r: bytes[j],
        g: bytes[j+1],
        b: bytes[j+2],
      });
    }

    Ok(out)
  }

  /// Create a new ColorPalette point.
  pub fn new(r: u8, g: u8, b: u8) -> ColorPalette {
    ColorPalette {
      r,
      g,
      b
    }
  }
}

/// 3D Coordinates with True Color (format 4)
#[derive(Clone, Debug, Default)]
pub struct TrueColorPoint3d {
  /// X coordinate
  pub x: i16,
  /// Y coordinate
  pub y: i16,
  /// Z coordinate
  pub z: i16,
  /// Last point bit and blanking bit.
  pub status_code: u8,
  /// Blue
  pub b: u8,
  /// Green
  pub g: u8,
  /// Red
  pub r: u8,
}

impl TrueColorPoint3d {
  /// Read multiple `TrueColorPoint3d` from raw bytes.
  pub fn read_bytes(bytes: &[u8])
      -> Result<Vec<TrueColorPoint3d>, IldaError> {
    if bytes.len() % TRUE_COLOR_3D_DATA_SIZE != 0 {
      return Err(IldaError::InvalidData);
    }

    let size = bytes.len() / TRUE_COLOR_3D_DATA_SIZE;
    let mut out = Vec::with_capacity(size);

    for i in 0..size {
      let j = i * TRUE_COLOR_3D_DATA_SIZE;
      out.push(TrueColorPoint3d {
        x: read_i16(&bytes[j .. j+2]),
        y: read_i16(&bytes[j+2 .. j+4]),
        z: read_i16(&bytes[j+4 .. j+6]),
        status_code: bytes[j+6],
        b: bytes[7],
        g: bytes[8],
        r: bytes[9],
      });
    }

    Ok(out)
  }

  /// Whether the point is a blanking point.
  pub fn is_blank(&self) -> bool {
    // 7th high order bit is the blanking bit.
    self.status_code & 64 == 64
  }

  /// Create a new TrueColorPoint2d point.
  pub fn new(x: i16, y: i16, z: i16, r: u8, g: u8, b: u8, is_last: bool, is_blank: bool) -> TrueColorPoint3d {
    TrueColorPoint3d {
      x,
      y,
      z,
      status_code: build_status_code(is_blank, is_last),
      r,
      g,
      b
    }
  }
}

/// 3D Coordinates with True Color (format 5)
#[derive(Clone, Debug, Default)]
pub struct TrueColorPoint2d {
  /// X coordinate
  pub x: i16,
  /// Y coordinate
  pub y: i16,
  /// Last point bit and blanking bit.
  pub status_code: u8,
  /// Blue
  pub b: u8,
  /// Green
  pub g: u8,
  /// Red
  pub r: u8,
}

impl TrueColorPoint2d {
  /// Read multiple `TrueColorPoint2d` from raw bytes.
  pub fn read_bytes(bytes: &[u8])
      -> Result<Vec<TrueColorPoint2d>, IldaError> {
    if bytes.len() % TRUE_COLOR_2D_DATA_SIZE != 0 {
      return Err(IldaError::InvalidData);
    }

    let size = bytes.len() / TRUE_COLOR_2D_DATA_SIZE;
    let mut out = Vec::with_capacity(size);

    for i in 0..size {
      let j = i * TRUE_COLOR_2D_DATA_SIZE;
      out.push(TrueColorPoint2d {
        x: read_i16(&bytes[j .. j+2]),
        y: read_i16(&bytes[j+2 .. j+4]),
        status_code: bytes[j+4],
        b: bytes[j+5],
        g: bytes[j+6],
        r: bytes[j+7],
      });
    }

    Ok(out)
  }

  /// Whether the point is a blanking point.
  pub fn is_blank(&self) -> bool {
    // 7th high order bit is the blanking bit.
    self.status_code & 64 == 64
  }

  /// Create a new TrueColorPoint2d point.
  pub fn new(x: i16, y: i16, r: u8, g: u8, b: u8, is_last: bool, is_blank: bool) -> TrueColorPoint2d {
    TrueColorPoint2d {
      x,
      y,
      status_code: build_status_code(is_blank, is_last),
      r,
      g,
      b
    }
  }
}

/// ILDA header and data records.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum IldaEntry {
  HeaderEntry(Header),
  TcPoint3dEntry(TrueColorPoint3d),
  TcPoint2dEntry(TrueColorPoint2d),
  ColorPaletteEntry(ColorPalette),
  IdxPoint3dEntry(IndexedPoint3d),
  IdxPoint2dEntry(IndexedPoint2d),
}

// FIXME:
// Reads in as little endian from big endian source. Not cross-platform.
fn read_i16(bytes: &[u8]) -> i16 {
  (((bytes[0] as u16) << 8) | (bytes[1] as u16)) as i16
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_indexed_2d_blanking_bit() {
    let mut point = IndexedPoint2d::default();
    point.status_code = 0;
    assert_eq!(false, point.is_blank());
    point.status_code = 128;
    assert_eq!(false, point.is_blank());
    point.status_code = 64;
    assert_eq!(true, point.is_blank());
    point.status_code = 255;
    assert_eq!(true, point.is_blank());
  }

  #[test]
  fn test_indexed_3d_blanking_bit() {
    let mut point = IndexedPoint3d::default();
    point.status_code = 0;
    assert_eq!(false, point.is_blank());
    point.status_code = 128;
    assert_eq!(false, point.is_blank());
    point.status_code = 64;
    assert_eq!(true, point.is_blank());
    point.status_code = 255;
    assert_eq!(true, point.is_blank());
  }

  #[test]
  fn test_truecolor_2d_blanking_bit() {
    let mut point = TrueColorPoint2d::default();
    point.status_code = 0;
    assert_eq!(false, point.is_blank());
    point.status_code = 128;
    assert_eq!(false, point.is_blank());
    point.status_code = 64;
    assert_eq!(true, point.is_blank());
    point.status_code = 255;
    assert_eq!(true, point.is_blank());
  }

  #[test]
  fn test_truecolor_3d_blanking_bit() {
    let mut point = TrueColorPoint3d::default();
    point.status_code = 0;
    assert_eq!(false, point.is_blank());
    point.status_code = 128;
    assert_eq!(false, point.is_blank());
    point.status_code = 64;
    assert_eq!(true, point.is_blank());
    point.status_code = 255;
    assert_eq!(true, point.is_blank());
  }
}
