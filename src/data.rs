// Copyright (c) 2016 Brandon Thomas <bt@brand.io>

// Coordinate data
// 33 - 34    X coord, signed 2's compliment, -32768, +32767
// 35 - 36    Y coord, signed 2's compliment, -32768, +32767
// 37 - 38    Z coord, signed 2's compliment, -32768, +32767
// 39 - 40    Status code

/*
  Processing pipeline:

    read(file) -> entries: Vec<IldaEntry> (which should be a direct in-memory mapping of the ILDA bytes)

  Where IldaEntry = { RawHeader, Tc2d, Tc3d, I2d, I3d, Palette }

  Then,

        process_frames(entries) -> Vec<Frame>

  Where Frame = { Frame2d, Frame3d }

  And,

    Frame2d {
        points: Vec<Point2d>
    }

    Point2d { x, y, r, g, b }

*/

const HEADER_SIZE : usize = 32;
const COLOR_PALETTE_SIZE: usize = 3;
const INDEXED_2D_DATA_SIZE: usize = 6;
const INDEXED_3D_DATA_SIZE: usize = 8;
const TRUE_COLOR_2D_DATA_SIZE: usize = 8;
const TRUE_COLOR_3D_DATA_SIZE: usize = 10;

pub enum ReadError {
  /// The wrong number of bytes has been supplied for reading.
  WrongSize,
}

/// The payload encoding formats currently supported.
pub enum Format {
  Unknown,
  ColorPalette,
  Indexed2d,
  Indexed3d,
  TrueColor2d,
  TrueColor3d,
}

// TODO: Name `Header`
/// A Raw ILDA header.
#[derive(Clone, Debug)]
pub struct RawHeader {
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

impl RawHeader {
  /// Returns the format of the header.
  pub fn get_format(&self) -> Format {
    match self.format_code {
      0u8 => Format::Indexed3d,
      1u8 => Format::Indexed2d,
      2u8 => Format::ColorPalette,
      4u8 => Format::TrueColor3d,
      5u8 => Format::TrueColor2d,
      _ => Format::Unknown,
    }
  }
}

/// 3D Coordinates with Indexed Color (format 0)
#[derive(Clone, Debug)]
pub struct IndexedPoint3d {
  pub x: i16,
  pub y: i16,
  pub z: i16,
  pub status_code: i8,
  pub color_index: i8,
}

impl IndexedPoint3d {
  /// Read an `IndexedPoint3d` from raw bytes.
  pub fn from_bytes(bytes: &[u8]) -> Result<IndexedPoint3d, ReadError> {
    if bytes.len() != INDEXED_3D_DATA_SIZE {
      return Err(ReadError::WrongSize);
    }
    Ok(IndexedPoint3d {
      x: read_i16(&bytes[0..1]),
      y: read_i16(&bytes[2..3]),
      z: read_i16(&bytes[4..5]),
      status_code: bytes[6] as i8,
      color_index: bytes[7] as i8,
    })
  }

  /// Read multiple `IndexedPoint3d` from raw bytes.
  pub fn read_bytes(bytes: &[u8]) -> Result<Vec<IndexedPoint3d>, ReadError> {
    if bytes.len() % INDEXED_3D_DATA_SIZE != 0 {
      return Err(ReadError::WrongSize);
    }

    let size = bytes.len() / INDEXED_3D_DATA_SIZE;
    let mut out = Vec::with_capacity(size);

    for i in 0..size {
      out.push(IndexedPoint3d {
        x: read_i16(&bytes[0..2]),
        y: read_i16(&bytes[2..4]),
        z: read_i16(&bytes[4..6]),
        status_code: bytes[6] as i8,
        color_index: bytes[7] as i8,
      });
    }

    Ok(out)
  }
}

/// 2D Coordinates with Indexed Color (format 1)
#[derive(Clone, Debug)]
pub struct IndexedPoint2d {
  pub x: i16,
  pub y: i16,
  pub status_code: i8,
  pub color_index: i8,
}

/// Color Palette (format 2)
#[derive(Clone, Debug)]
pub struct ColorPalette {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

/// 3D Coordinates with True Color (format 4)
#[derive(Clone, Debug)]
pub struct TrueColorPoint3d {
  pub x: i16,
  pub y: i16,
  pub z: i16,
  pub status_code: i8,
  pub b: u8,
  pub g: u8,
  pub r: u8,
}

impl TrueColorPoint3d {
  /// Read a single `TrueColorPoint3d` from raw bytes.
  pub fn from_bytes(bytes: &[u8]) -> Result<TrueColorPoint3d, ReadError> {
    if bytes.len() != TRUE_COLOR_3D_DATA_SIZE {
      return Err(ReadError::WrongSize);
    }
    Ok(TrueColorPoint3d {
      x: read_i16(&bytes[0..1]),
      y: read_i16(&bytes[2..3]),
      z: read_i16(&bytes[4..5]),
      status_code: bytes[6] as i8,
      b: bytes[7],
      g: bytes[8],
      r: bytes[9],
    })
  }

  /// Read multiple `TrueColorPoint3d` from raw bytes.
  pub fn read_bytes(bytes: &[u8]) -> Result<Vec<TrueColorPoint3d>, ReadError> {
    if bytes.len() % TRUE_COLOR_3D_DATA_SIZE != 0 {
      return Err(ReadError::WrongSize);
    }

    let size = bytes.len() / TRUE_COLOR_3D_DATA_SIZE;
    let mut out = Vec::with_capacity(size);

    for i in 0..size {
      out.push(TrueColorPoint3d {
        x: read_i16(&bytes[0..1]),
        y: read_i16(&bytes[2..3]),
        z: read_i16(&bytes[4..5]),
        status_code: bytes[6] as i8,
        b: bytes[7],
        g: bytes[8],
        r: bytes[9],
      });
    }

    Ok(out)
  }
}

/// 3D Coordinates with True Color (format 5)
#[derive(Clone, Debug)]
pub struct TrueColorPoint2d {
  pub x: i16,
  pub y: i16,
  pub z: i16,
  pub b: u8,
  pub g: u8,
  pub r: u8,
}

#[derive(Clone, Debug)]
pub enum IldaEntry {
  HeaderEntry(RawHeader),
  TcPoint3dEntry(TrueColorPoint3d),
  TcPoint2dEntry(TrueColorPoint2d),
  ColorPaletteEntry(ColorPalette),
  IdxPoint3dEntry(IndexedPoint3d),
  IdxPoint2dEntry(IndexedPoint2d),
}

// TODO: DE-DUPLICATE
// TODO/FIXME: Does Rust's casting use 2's complement? Do some maths.
fn read_i16(bytes: &[u8]) -> i16 {
  (((bytes[0] as u16) << 8) | (bytes[1] as u16)) as i16
}

fn read_u16(bytes: &[u8]) -> u16 {
  ((bytes[0] as u16) << 8) | (bytes[1] as u16)
}
