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

// TODO: Name `Header`
/// A Raw ILDA header.
#[derive(Debug)]
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

/// 3D Coordinates with Indexed Color (format 0)
#[derive(Debug)]
pub struct IndexedPoint3d {
  pub x: i16,
  pub y: i16,
  pub z: i16,
  pub status_code: i8,
  pub color_index: i8,
}

/// 2D Coordinates with Indexed Color (format 1)
#[derive(Debug)]
pub struct IndexedPoint2d {
  pub x: i16,
  pub y: i16,
  pub status_code: i8,
  pub color_index: i8,
}

/// Color Palette (format 2)
#[derive(Debug)]
pub struct ColorPalette {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

/// 3D Coordinates with True Color (format 4)
#[derive(Debug)]
pub struct TrueColorPoint3d {
  pub x: i16,
  pub y: i16,
  pub z: i16,
  pub status_code: i8,
  pub b: u8,
  pub g: u8,
  pub r: u8,
}

/// 3D Coordinates with True Color (format 5)
#[derive(Debug)]
pub struct TrueColorPoint2d {
  pub x: i16,
  pub y: i16,
  pub z: i16,
  pub b: u8,
  pub g: u8,
  pub r: u8,
}

#[derive(Debug)]
pub enum IldaEntry {
  HeaderEntry(RawHeader),
  TcPoint3dEntry(TrueColorPoint3d),
  TcPoint2dEntry(TrueColorPoint2d),
  ColorPaletteEntry(ColorPalette),
  IdxPoint3dEntry(IndexedPoint3d),
  IdxPoint2dEntry(IndexedPoint2d),
}

