// Copyright (c) 2015-2016 Brandon Thomas <bt@brand.io>

//! This module presents a higher-level representation of data read from ILDA
//! files, organizing the data into "frames". Frames contain points. It's a
//! simple representation that doesn't expose color palettes, indexed colors,
//! and so forth.

use color::default_color_index;
use data::Format;
use data::Header;
use data::IldaEntry;
use data::TrueColorPoint2d;
use error::IldaError;
use parser::stream_with_error;
use parser::IldaEntryIteratorWithError;
use point::SimplePoint;
use std::fs::File;
use std::io::Cursor;
use std::io::{Read, Write};
use std::vec::IntoIter;
use writer::IldaWriter;

/// An animation is comprised of one or more frames.
#[derive(Clone)]
pub struct Animation {
  frames: Vec<Frame>,
}

/// A single frame of animation, comprised of many points.
#[derive(Clone)]
pub struct Frame {
  points: Vec<SimplePoint>,
  frame_name: Option<String>,
  company_name: Option<String>,
}

/// Output ILDA frames into a Writer that implements Write
pub struct AnimationStreamWriter<T> where T: Write {
    inner: IldaWriter<T>,
    finalized: bool,
}

impl<W: Write> AnimationStreamWriter<W> {
  /// Create a new AnimationStreamWriter instance
  pub fn new(inner: W) -> AnimationStreamWriter<W> {
    AnimationStreamWriter {
      inner: IldaWriter::new(inner),
      finalized: false
    }
  }

  /// Write a frame into the stream. (Simple version for pseudo streaming)
  /// This will set the frame number and total_frames value of the header to 0.
  /// To specify these values, use `write_frame_ext` instead.
  pub fn write_frame(&mut self, frame: &Frame) -> Result<(), IldaError> {
    self.write_frame_ext(frame, 0, 0)
  }

  /// Write a frame into the stream.
  pub fn write_frame_ext(&mut self, frame: &Frame, number: u16, total_frames: u16) -> Result<(), IldaError> {
    let len = frame.points.len();

    if len > u16::max_value() as usize {
      return Err(IldaError::TooManyPoints(len));
    }

    let header = Header::new(Format::TrueColor2d, frame.frame_name.clone(), frame.company_name.clone(), len as u16, number, total_frames, 0);

    self.inner.write(IldaEntry::HeaderEntry(header))?;

    for (i, point) in frame.points.iter().enumerate() {
      let ilda_point = TrueColorPoint2d::new(point.x, point.y, point.r, point.g, point.b, i + 1 == len, point.is_blank);
      self.inner.write(IldaEntry::TcPoint2dEntry(ilda_point))?
    }

    Ok(())
  }

  fn write_finishing_header(&mut self) -> Result<(), IldaError> {
    let header = Header::new(Format::TrueColor2d, None, None, 0, 0, 0, 0);

    self.inner.write(IldaEntry::HeaderEntry(header))
  }

  /// Consume the writer and finish the ILDA file by writing the final header.
  /// If this method is not called, Drop will take care of writing the final header.
  /// Therefore calling this function is optional but recommended to catch possible errors.
  pub fn finalize(mut self) -> Result<(), IldaError> {
    self.finalized = true;
    self.write_finishing_header()
  }
}

impl<W: Write> Drop for AnimationStreamWriter<W> {
  fn drop(&mut self) {
    if !self.finalized {
      self.write_finishing_header().unwrap();
    }
  }
}

/// Iterator over animation Frame items. Panics on error.
pub struct AnimationFrameIterator<'a>(IldaEntryIteratorWithError<'a>);

/// Iterator over animation Result<Frame, IldaError> items.
pub struct AnimationFrameIteratorWithError<'a>(IldaEntryIteratorWithError<'a>);

impl Animation {
  /// Creates a new animation from frames.
  pub fn new(frames: Vec<Frame>) -> Animation {
    Animation { frames }
  }

  /// Read an animation from an ILDA file.
  ///
  /// ```
  /// # use ilda::animation::Animation;
  /// let filename = "examples/files/ildatest.ild";
  /// let animation = Animation::read_file(filename).unwrap();
  ///
  /// assert_eq!(1, animation.frame_count());
  /// ```
  pub fn read_file(filename: &str) -> Result<Animation, IldaError> {
    let mut file = File::open(filename)?;
    let iter = Self::stream_with_error(&mut file);
    let result: Result<Vec<Frame>, IldaError> = iter.collect();
    Ok(Animation { frames: result? })
  }

  /// Read an animation from raw ILDA bytes.
  pub fn read_bytes(ilda_bytes: &[u8]) -> Result<Animation, IldaError> {
    let mut cursor = Cursor::new(ilda_bytes);
    let iter = Self::stream_with_error(&mut cursor);
    let result: Result<Vec<Frame>, IldaError> = iter.collect();
    Ok(Animation { frames: result? })
  }

  /// Stream Animation Frames from a reader
  pub fn stream(ilda_reader: &mut Read) -> AnimationFrameIterator {
    let parser_iter = stream_with_error(ilda_reader);
    AnimationFrameIterator(parser_iter)
  }

  /// Stream Animation Frames (with error handling) from a reader
  pub fn stream_with_error(ilda_reader: &mut Read) -> AnimationFrameIteratorWithError {
    let parser_iter = stream_with_error(ilda_reader);
    AnimationFrameIteratorWithError(parser_iter)
  }

  /// Write Animation to a file
  pub fn write_file(&self, filename: &str) -> Result<(), IldaError> {
    let mut file = File::create(filename)?;
    self.write(&mut file)
  }

  /// Write Animation to Writer
  pub fn write<T>(&self, writer: T) -> Result<(), IldaError> where T: Write {
    let mut streamer = AnimationStreamWriter::new(writer);
    let len = self.frames.len();
    if len > u16::max_value() as usize {
      return Err(IldaError::TooManyFrames(len));
    }

    for (i, frame) in self.frames.iter().enumerate() {
      streamer.write_frame_ext(frame, i as u16, len as u16)?;
    }

    streamer.finalize()
  }

  /// Get an frame iterator for the animation.
  pub fn into_frame_iter(self) -> IntoIter<Frame> {
    self.frames.into_iter()
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
}

fn next_frame(iter: &mut IldaEntryIteratorWithError) -> Result<Option<Frame>, IldaError> {
  let entry = match iter.next().transpose()? {
    Some(entry) => entry,
    None => return Ok(None), // no more data
  };

  let mut points_to_read;

  let mut frame = match entry {
    IldaEntry::HeaderEntry(mut header) => {
      points_to_read = header.record_count;
      Frame {
        points: Vec::new(),
        frame_name: header.name.take(),
        company_name: header.company_name.take(),
      }
    }
    _ => return Err(IldaError::InvalidData), // expected header
  };

  if points_to_read == 0 {
    // EOF header
    return Ok(None);
  }

  while points_to_read > 0 {
    let entry = match iter.next().transpose()? {
      Some(entry) => entry,
      None => return Err(IldaError::InvalidData), // premature end of stream
    };

    points_to_read = points_to_read - 1;

    let point = ilda_entry_to_point(entry)?;
    frame.points.push(point);
  }

  Ok(Some(frame))
}

impl<'a> Iterator for AnimationFrameIterator<'a> {
  type Item = Frame;

  fn next(&mut self) -> Option<Self::Item> {
    next_frame(&mut self.0).unwrap()
  }
}

impl<'a> Iterator for AnimationFrameIteratorWithError<'a> {
  type Item = Result<Frame, IldaError>;

  fn next(&mut self) -> Option<Self::Item> {
    next_frame(&mut self.0).transpose()
  }
}


/// Convert an IldaEntry containing a point into a respective animation point.
/// Color palettes and headers will return errors.
pub fn ilda_entry_to_point(entry: IldaEntry) -> Result<SimplePoint, IldaError> {
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
      Ok(SimplePoint {
        x: point.x,
        y: point.y,
        r: point.r,
        g: point.g,
        b: point.b,
        is_blank: point.is_blank(),
      })
    },
    IldaEntry::TcPoint3dEntry(point) => {
      Ok(SimplePoint {
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
      Ok(SimplePoint {
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
      Ok(SimplePoint {
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
  /// Create a new frame from points.
  pub fn new(points: Vec<SimplePoint>, frame_name: Option<String>, company_name: Option<String>) -> Frame {
    Frame { points, frame_name, company_name }
  }

  /// Get a reference to the points in the frame.
  pub fn get_points(&self) -> &Vec<SimplePoint> {
    &self.points
  }

  /// Get the number of points in the frame.
  pub fn point_count(&self) -> usize {
    self.points.len()
  }

  /// Get a reference to the point at the given offset, if it exists.
  pub fn get_point(&self, position: usize) -> Option<&SimplePoint> {
    self.points.get(position)
  }
}

/// Iterator over all the points from all of the frames in the animation.
#[derive(Clone, Copy)]
pub struct AnimationPointIterator<'a> {
  animation: &'a Animation,
  current_frame: Option<&'a Frame>, // Iteration ends when None.
  frame_index: usize,
  point_index: usize,
}

impl <'a> AnimationPointIterator<'a> {
  // Get the next point for the current frame and advance pointer.
  fn next_point_for_frame(&mut self) -> Option<&'a SimplePoint> {
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
  type Item = &'a SimplePoint;

  fn into_iter(self) -> Self::IntoIter {
    FramePointIterator { frame: self, index: 0 }
  }
}

impl<'a> Iterator for AnimationPointIterator<'a> {
  type Item = &'a SimplePoint;

  fn next(&mut self) -> Option<Self::Item> {
    self.next_point_for_frame().or_else(|| {
      self.next_frame();
      self.next_point_for_frame()
    })
  }
}

impl<'a> Iterator for FramePointIterator<'a> {
  type Item = &'a SimplePoint;

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
        points.push(SimplePoint::default());
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
  fn point(color: u8) -> SimplePoint {
    SimplePoint {
      x: 0,
      y: 0,
      r: color,
      g: color,
      b: color,
      is_blank: false,
    }
  }

  // CTOR.
  fn frame(points: Vec<SimplePoint>) -> Frame {
    Frame {
      points: points,
      frame_name: None,
      company_name: None,
    }
  }
}
