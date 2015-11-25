// Copyright (c) 2015 Brandon Thomas <bt@brand.io>

pub struct IndexedPoint {
  x: i16,
  y: i16,
  z: i16,
  isLastPoint: bool,
  /// If the laser should treat this as a blanking point.
  isBlank: bool,
  colorIndex: u8,
}

pub struct TrueColorPoint {
  x: i16,
  y: i16,
  z: i16,
  isLastPoint: bool,
  /// If the laser should treat this as a blanking point.
  isBlank: bool,
  r: u8,
  g: u8,
  b: u8,
}


pub struct IndexedFrame {
  frameName: String,
  companyName: String,
  frameNumber: u16,
  /// The number of frames in this sequence.
  totalFrames: u16,
  is3d: bool,
  points: Vec<IndexedPoint>,
}

pub struct TrueColorFrame {
  frameName: String,
  companyName: String,
  frameNumber: u16,

  /// The number of frames in this sequence.
  totalFrames: u16,
  is3d: bool,

  points: Vec<TrueColorPoint>,
}

pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

pub struct ColorPalette {
  paletteName: String,
  companyName: String,
  paletteNumber: u16, // TODO: Used?
  projectorNumber: u8, // TODO: Used?
  colors: Vec<Color>,
}

