// Copyright (c) 2016 Brandon Thomas <bt@brand.io>

// Coordinate data
// 33 - 34    X coord, signed 2's compliment, -32768, +32767
// 35 - 36    Y coord, signed 2's compliment, -32768, +32767
// 37 - 38    Z coord, signed 2's compliment, -32768, +32767
// 39 - 40    Status code

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

