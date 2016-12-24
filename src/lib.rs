// Copyright (c) 2015 Brandon Thomas <bt@brand.io>

/*
TODO

 - "idtf" reads raw stream of headers, byte payloads, etc.
 - "abstract" reads data into abstract "frame" and "color"
 - There's a lot of dead code here.
*/

pub mod ilda {
  pub use idtf;
  pub use data;
  pub use limit;
  pub use animation;
}


// Per frame
// 1-4      "I", "L", "D", "A"
// 5-7      0, 0, 0
// 8        0 - 3D (4 words per point), 1 - 2D (3 words per point)
// 9 - 16   Name of the frame
// 17 - 24  Name of the company that made the frame
// 25 - 26  Total number of pts in this image (1-65535), if 0 end of file
// 27 - 28  If part of a frame group
// 29 - 30  Total number of frames in frame group
// 31       Scanner head index (0-255)
// 32       Reserved for future use, must be 0
pub fn parse_frame() {
}

// Coordinate data
// 33 - 34    X coord, signed 2's compliment, -32768, +32767
// 35 - 36    Y coord, signed 2's compliment, -32768, +32767
// 37 - 38    Z coord, signed 2's compliment, -32768, +32767
// 39 - 40    Status code
pub fn parse_coordinates() {
}

pub enum Format {
  Indexed3d, // 0
  Indexed2d, // 1
  ColorPalette, // 2
  TrueColor3d, // 4
  TrueColor2d, // 5
}

mod error;
mod color;

pub mod data;
pub mod idtf;
pub mod limit;
pub mod parser;
pub mod animation;

pub use error::IldaError;
