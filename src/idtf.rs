// Copyright (c) 2015 Brandon Thomas <bt@brand.io>

/// A point with an asigned RGB color.
pub struct TrueColorPoint {
  x: i16,
  y: i16,
  z: i16,
  /// Whether this is the last point in the image.
  is_last_point: bool,
  /// If the laser should treat this as a blanking point.
  is_blank: bool,
  r: u8,
  g: u8,
  b: u8,
}

/// A point with a color palette lookup index.
pub struct IndexedPoint {
  x: i16,
  y: i16,
  z: i16,
  /// Whether this is the last point in the image.
  is_last_point: bool,
  /// If the laser should treat this as a blanking point.
  is_blank: bool,
  color_index: u8,
}

/// A color within a `Header::ColorPalette`.
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

pub enum Header {
  /// A 2D or 3D frame where each point is assigned an RGB color.
  TrueColorFrame {
    frame_name: Option<String>,
    company_name: Option<String>,
    /// The projector to display this frame on.
    projector_number: u8,
    frame_number: u16,
    /// The number of frames in this sequence.
    total_frames: u16,
    /// Whether the z-coordinate is used.
    is_3d: bool,
    points: Vec<TrueColorPoint>,
  },

  /// A 2D or 3D frame with indexed colors.
  IndexedFrame {
    frame_name: Option<String>,
    company_name: Option<String>,
    /// The projector to display this frame on.
    projector_number: u8,
    frame_number: u16,
    /// The number of frames in this sequence.
    total_frames: u16,
    /// Whether the z-coordinate is used.
    is_3d: bool,
    points: Vec<IndexedPoint>,
  },

  /// A color palette that is used for IndexedFrames/IndexedPoints.
  ColorPalette {
    palette_name: Option<String>,
    company_name: Option<String>,
    palette_number: u16, // TODO: Used?
    projector_number: u8, // TODO: Used?
    colors: Vec<Color>,
  },
}

