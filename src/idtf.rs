// Copyright (c) 2015 Brandon Thomas <bt@brand.io>

// TODO: None of this seems to be in use.

/// A point with an asigned RGB color.
#[derive(Debug)]
pub struct TrueColorPoint {
  pub x: i16,
  pub y: i16,
  pub z: i16,
  /// Whether this is the last point in the image.
  pub is_last_point: bool,
  /// If the laser should treat this as a blanking point.
  pub is_blank: bool,
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

/// A point with a color palette lookup index.
#[derive(Debug)]
pub struct IndexedPoint {
  pub x: i16,
  pub y: i16,
  pub z: i16,
  /// Whether this is the last point in the image.
  pub is_last_point: bool,
  /// If the laser should treat this as a blanking point.
  pub is_blank: bool,
  pub color_index: u8,
}

/// A color within a `Header::ColorPalette`.
#[derive(Debug)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

// TODO: De-enumify this.
#[derive(Debug)]
pub enum OldHeader {
  /// A 2D or 3D frame where each point is assigned an RGB color.
  TrueColorFrame {
    frame_name: Option<String>,
    company_name: Option<String>,
    /// The number of points records following this header.
    /// If 0, this is the EOF header.
    records: u16,
    /// If the frame is part of an animation, this is the frame number
    /// within the animation sequence.
    frame_number: u16,
    /// The number of frames within this sequence.
    total_frames: u16,
    /// Whether the z-coordinate is used.
    is_3d: bool,
    /// The projector to display this frame on.
    projector_number: u8,
    points: Vec<TrueColorPoint>,
  },

  /// A 2D or 3D frame with indexed colors.
  IndexedFrame {
    frame_name: Option<String>,
    company_name: Option<String>,
    /// The number of points records following this header.
    /// If 0, this is the EOF header.
    records: u16,
    /// If the frame is part of an animation, this is the frame number
    /// within the animation sequence.
    frame_number: u16,
    /// The number of frames within this sequence.
    total_frames: u16,
    /// Whether the z-coordinate is used.
    is_3d: bool,
    /// The projector to display this frame on.
    projector_number: u8,
    points: Vec<IndexedPoint>,
  },

  /// A color palette that is used for IndexedFrames/IndexedPoints.
  ColorPalette {
    palette_name: Option<String>,
    company_name: Option<String>,
    /// The number of color records following this header.
    /// Must be within the range [2, 256].
    /// If 0, this is the EOF header.
    records: u16,
    palette_number: u16, // TODO: Used?
    projector_number: u8, // TODO: Used?
    colors: Vec<Color>,
  },
}

