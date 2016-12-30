// Copyright (c) 2015-2016 Brandon Thomas <bt@brand.io>

//! This module presents a higher-level representation of data read from ILDA
//! files, organizing the data into "frames". Frames contain points. It's a
//! simple representation that doesn't expose color palettes, indexed colors,
//! and so forth.

use color::default_color_index;
use data::IldaEntry;
use error::IldaError;
use parser::read_bytes;
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
#[derive(Clone, Debug, Default)]
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
  /// If the laser should treat this as a blanking point.
  pub is_blank: bool,
}

impl Animation {
  /// Read an animation from an ILDA file.
  ///
  /// ```
  /// # use ilda::animation::Animation;
  /// let filename = "examples/files/ildatest.ild";
  /// let animation = Animation::read_file(filename).unwrap();
  ///
  /// assert_eq!(2, animation.frame_count());
  /// ```
  pub fn read_file(filename: &str) -> Result<Animation, IldaError> {
    let entries = read_file(filename)?;
    Animation::process_entries(entries)
  }

  /// Read an animation from raw ILDA bytes.
  pub fn read_bytes(ilda_bytes: &[u8]) -> Result<Animation, IldaError> {
    let entries = read_bytes(ilda_bytes)?;
    Animation::process_entries(entries)
  }

  /// Get an frame iterator for the animation.
  pub fn into_frame_iter<'a>(&'a self) -> AnimationFrameIterator<'a> {
    AnimationFrameIterator { animation: self, index: 0 }
  }

  /// Get a point iterator for the animation, which will iterate over all points
  /// from all frames.
  pub fn into_point_iter<'a>(&'a self) -> AnimationPointIterator<'a> {
    AnimationPointIterator {
      animation: self,
      current_frame: self.frames.get(0),
      frame_index: 0,
      point_index: 0,
    }
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

  fn process_entries(entries: Vec<IldaEntry>) -> Result<Animation, IldaError> {
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

          continue;
        },
        _ => {},
      }

      let mut frame = match current_frame {
        // TODO: Better error type / message
        None => return Err(IldaError::InvalidData),
        Some(ref mut frame) => frame,
      };

      let point = ilda_entry_to_point(entry)?;
      frame.points.push(point);
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
}

/// Convert an IldaEntry containing a point into a respective animation point.
/// Color palettes and headers will return errors.
pub fn ilda_entry_to_point(entry: IldaEntry) -> Result<Point, IldaError> {
  match entry {
    IldaEntry::HeaderEntry(_) => {
      // Already handled by caller.
      Err(IldaError::InvalidData)
    },
    IldaEntry::ColorPaletteEntry(_) => {
      // TODO: Handle color palettes.
      Err(IldaError::Unsupported)
    },
    IldaEntry::TcPoint2dEntry(point) => {
      Ok(Point {
        x: point.x,
        y: point.y,
        r: point.r,
        g: point.g,
        b: point.b,
        is_blank: point.is_blank(),
      })
    },
    IldaEntry::TcPoint3dEntry(point) => {
      Ok(Point {
        x: point.x,
        y: point.y,
        r: point.r,
        g: point.g,
        b: point.b,
        is_blank: point.is_blank(),
      })
    },
    IldaEntry::IdxPoint2dEntry(point) => {
      let color = default_color_index(point.color_index);
      Ok(Point {
        x: point.x,
        y: point.y,
        r: color.r,
        g: color.g,
        b: color.b,
        is_blank: point.is_blank(),
      })
    },
    IldaEntry::IdxPoint3dEntry(point) => {
      let color = default_color_index(point.color_index);
      Ok(Point {
        x: point.x,
        y: point.y,
        r: color.r,
        g: color.g,
        b: color.b,
        is_blank: point.is_blank(),
      })
    },
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

/// Iterator over all the frames in the animation.
pub struct AnimationFrameIterator<'a> {
  animation: &'a Animation,
  index: usize,
}

/// Iterator over all the points from all of the frames in the animation.
pub struct AnimationPointIterator<'a> {
  animation: &'a Animation,
  current_frame: Option<&'a Frame>, // Iteration ends when None.
  frame_index: usize,
  point_index: usize,
}

impl <'a> AnimationPointIterator<'a> {
  // Get the next point for the current frame and advance pointer.
  fn next_point_for_frame(&mut self) -> Option<&'a Point> {
    match self.current_frame {
      None => return None, // Iteration has ended
      Some(frame) => {
        match frame.get_point(self.point_index) {
          Some(point) => {
            self.point_index += 1;
            Some(point)
          },
          None => None,
        }
      },
    }
  }

  // Get the next frame and advance pointer.
  fn next_frame(&mut self) -> Option<&'a Frame> {
    self.frame_index += 1;
    self.point_index = 0;
    self.current_frame = self.animation.get_frame(self.frame_index);
    self.current_frame
  }
}

/// Iterator over all of the points in a single frame.
pub struct FramePointIterator<'a> {
  frame: &'a Frame,
  index: usize,
}

impl<'a> IntoIterator for &'a Frame {
  type IntoIter = FramePointIterator<'a>;
  type Item = &'a Point;

  fn into_iter(self) -> Self::IntoIter {
    FramePointIterator { frame: self, index: 0 }
  }
}

impl<'a> Iterator for AnimationFrameIterator<'a> {
  type Item = &'a Frame;

  fn next(&mut self) -> Option<Self::Item> {
    let item = self.animation.get_frame(self.index);
    self.index += 1;
    item
  }
}

impl<'a> Iterator for AnimationPointIterator<'a> {
  type Item = &'a Point;

  fn next(&mut self) -> Option<Self::Item> {
    self.next_point_for_frame().or_else(|| {
      self.next_frame();
      self.next_point_for_frame()
    })
  }
}

impl<'a> Iterator for FramePointIterator<'a> {
  type Item = &'a Point;

  fn next(&mut self) -> Option<Self::Item> {
    let item = self.frame.get_point(self.index);
    self.index += 1;
    item
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use data::IldaEntry;
  use data::IndexedPoint2d;
  use data::TrueColorPoint2d;

  #[test]
  fn test_animation_frame_iterator() {
    // Create sentinel value frames.
    fn frame(num_points: usize) -> Frame {
      let mut points = Vec::new();
      for _i in 0..num_points {
        points.push(Point::default());
      }
      Frame {
        points: points,
        frame_name: None,
        company_name: None,
      }
    }

    let animation = Animation {
      frames: vec![frame(1), frame(2), frame(3)],
    };

    let mut iter = animation.into_frame_iter();

    let frame = iter.next();
    assert!(frame.is_some());
    assert_eq!(1, frame.unwrap().point_count());

    let frame = iter.next();
    assert!(frame.is_some());
    assert_eq!(2, frame.unwrap().point_count());

    let frame = iter.next();
    assert!(frame.is_some());
    assert_eq!(3, frame.unwrap().point_count());

    let frame = iter.next();
    assert!(frame.is_none());
  }

  #[test]
  fn test_animation_point_iterator() {
    let frame1 = frame(vec![point(0), point(1), point(2)]);
    let frame2 = frame(vec![point(3), point(4)]);
    let frame3 = frame(vec![point(5)]);
    let frame4 = frame(vec![point(6), point(7)]);

    let animation = Animation {
      frames: vec![frame1, frame2, frame3, frame4],
    };

    let values: Vec<_> = animation.into_point_iter()
        .map(|point| point.r)
        .collect();

    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(expected, values);
  }

  #[test]
  fn test_frame_point_iterator() {
    let frame = frame(vec![point(0), point(1), point(2), point(3), point(4)]);

    let values: Vec<_> = frame.into_iter()
        .map(|point| point.r)
        .collect();

    let expected = vec![0, 1, 2, 3, 4];
    assert_eq!(expected, values);
  }

  #[test]
  fn test_ilda_entry_to_point_true_color() {
    let ilda_point = TrueColorPoint2d::default();
    let entry = IldaEntry::TcPoint2dEntry(ilda_point);
    let point = ilda_entry_to_point(entry).unwrap();

    assert_eq!(point.r, 0);
    assert_eq!(point.g, 0);
    assert_eq!(point.b, 0);
    assert_eq!(point.x, 0);
    assert_eq!(point.y, 0);
    assert_eq!(point.is_blank, false);

    let mut ilda_point = TrueColorPoint2d::default();
    ilda_point.r = 255;
    ilda_point.g = 127;
    ilda_point.b = 32;
    ilda_point.x = 10_000;
    ilda_point.y = -10_000;
    ilda_point.status_code = 64;

    let entry = IldaEntry::TcPoint2dEntry(ilda_point);
    let point = ilda_entry_to_point(entry).unwrap();

    assert_eq!(point.r, 255);
    assert_eq!(point.g, 127);
    assert_eq!(point.b, 32);
    assert_eq!(point.x, 10_000);
    assert_eq!(point.y, -10_000);
    assert_eq!(point.is_blank, true);
  }

  #[test]
  fn test_ilda_entry_to_point_indexed() {
    let ilda_point = IndexedPoint2d::default();
    let entry = IldaEntry::IdxPoint2dEntry(ilda_point);
    let point = ilda_entry_to_point(entry).unwrap();

    assert_eq!(point.r, 255); // Red is on for indexed color "0"
    assert_eq!(point.g, 0);
    assert_eq!(point.b, 0);
    assert_eq!(point.x, 0);
    assert_eq!(point.y, 0);
    assert_eq!(point.is_blank, false);

    let mut ilda_point = IndexedPoint2d::default();
    ilda_point.x = 10_000;
    ilda_point.y = -10_000;
    ilda_point.status_code = 64;
    ilda_point.color_index = 57;

    let entry = IldaEntry::IdxPoint2dEntry(ilda_point);
    let point = ilda_entry_to_point(entry).unwrap();

    assert_eq!(point.r, 255);
    assert_eq!(point.g, 224);
    assert_eq!(point.b, 224);
    assert_eq!(point.x, 10_000);
    assert_eq!(point.y, -10_000);
    assert_eq!(point.is_blank, true);
  }

  // Create sentinel value points.
  fn point(color: u8) -> Point {
    Point {
      x: 0,
      y: 0,
      r: color,
      g: color,
      b: color,
      is_blank: false,
    }
  }

  // CTOR.
  fn frame(points: Vec<Point>) -> Frame {
    Frame {
      points: points,
      frame_name: None,
      company_name: None,
    }
  }
}
