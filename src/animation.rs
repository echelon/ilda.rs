// Copyright (c) 2015-2016 Brandon Thomas <bt@brand.io>

//! This module presents a higher-level representation of data read from ILDA
//! files, organizing the data into "frames". Frames contain points. It's a
//! simple representation that doesn't expose color palettes, indexed colors,
//! and so forth.

use data::IldaEntry;
use error::IldaError;
use parser::read_file;

/// An animation is comprised of one or more frames.
#[derive(Clone)]
pub struct Animation {
  frames: Vec<Frame>,
}

/// A single frame of animation, comprised of many points.
#[derive(Clone)]
pub struct Frame {
  points: Vec<Point>,
  frame_name: Option<String>,
  company_name: Option<String>,
}

/// A single coordinate point for the laser to draw.
#[derive(Clone, Debug)]
pub struct Point {
  /// X coordinate.
  pub x: i16,
  /// Y coordinate.
  pub y: i16,
  // TODO:
  // /// (Optional) Z coordinate.
  // pub z: i16,
  /// Red.
  pub r: u8,
  /// Green.
  pub g: u8,
  /// Blue.
  pub b: u8,
  // TODO:
  // /// Whether this is the last point in the image.
  // pub is_last_point: bool,
  // TODO:
  // /// If the laser should treat this as a blanking point.
  // pub is_blank: bool,
}

impl Animation {
  /// Read an animation from an ILDA file.
  pub fn read_file(filename: &str) -> Result<Animation, IldaError> {
    let entries = read_file(filename)?;

    let mut frames = Vec::new();
    let mut current_frame = None;

    // NB: This does not check for format consistency.
    // Frame-type / point-type mismatch is allowed.
    for entry in entries {
      match entry {
        IldaEntry::HeaderEntry(mut header) => {
          if current_frame.is_some() {
            let frame = current_frame.take().unwrap();
            frames.push(frame);
          }

          current_frame = Some(Frame {
            points: Vec::new(),
            frame_name: header.name.take(),
            company_name: header.company_name.take(),
          });
        },
        IldaEntry::TcPoint2dEntry(point) => {
          match current_frame {
            None => {
              // TODO: Better error type / message.
              return Err(IldaError::InvalidData);
            },
            Some(ref mut frame) => {
              frame.points.push(Point {
                x: point.x,
                y: point.y,
                r: point.r,
                g: point.g,
                b: point.b,
              });
            },
          }
        },
        _ => {
          // TODO: We only support one kind of frame for now. :(
          return Err(IldaError::Unsupported);
        },
      }
    }

    // Take the last frame.
    match current_frame.take() {
      None => {},
      Some(frame) => frames.push(frame),
    }

    if frames.is_empty() {
      return Err(IldaError::NoData);
    }

    Ok(Animation {
      frames: frames,
    })
  }

  /// Return a reference to the frames.
  pub fn get_frames(&self) -> &Vec<Frame> {
    &self.frames
  }

  /// Return the number of frames in the animation.
  pub fn frame_count(&self) -> usize {
    self.frames.len()
  }

  /// Get a reference to the frame at the given offset, if it exists.
  pub fn get_frame(&self, position: usize) -> Option<&Frame> {
    self.frames.get(position)
  }
}

impl Frame {
  /// Get a reference to the points in the frame.
  pub fn get_points(&self) -> &Vec<Point> {
    &self.points
  }

  /// Get the number of points in the frame.
  pub fn point_count(&self) -> usize {
    self.points.len()
  }

  /// Get a reference to the point at the given offset, if it exists.
  pub fn get_point(&self, position: usize) -> Option<&Point> {
    self.points.get(position)
  }
}

pub struct FrameIterator<'a> {
  animation: &'a Animation,
  index: usize,
}

pub struct PointIterator<'a> {
  frame: &'a Frame,
  index: usize,
}

impl <'a> IntoIterator for &'a Animation {
  type IntoIter = FrameIterator<'a>;
  type Item = &'a Frame;

  fn into_iter(self) -> Self::IntoIter {
    FrameIterator { animation: self, index: 0 }
  }
}

impl<'a> IntoIterator for &'a Frame {
  type IntoIter = PointIterator<'a>;
  type Item = &'a Point;

  fn into_iter(self) -> Self::IntoIter {
    PointIterator { frame: self, index: 0 }
  }
}

impl<'a> Iterator for FrameIterator<'a> {
  type Item = &'a Frame;

  fn next(&mut self) -> Option<Self::Item> {
    let item = self.animation.get_frame(self.index);
    self.index += 1;
    item
  }
}

impl<'a> Iterator for PointIterator<'a> {
  type Item = &'a Point;

  fn next(&mut self) -> Option<Self::Item> {
    let item = self.frame.get_point(self.index);
    self.index += 1;
    item
  }
}
