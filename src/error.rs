// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io;

/// Ilda library errors.
#[derive(Debug)]
pub enum IldaError {
  /// The ILDA file is too small to read.
  FileTooSmall,

  /// Problems were encountered while reading the ILDA data.
  InvalidData,

  /// Problems were encountered while reading the ILDA data, specifically with
  /// an invalid ILDA header section.
  InvalidHeader,

  /// Wraps standard library IO errors.
  IoError { cause: io::Error },
}

impl Error for IldaError {
  fn description(&self) -> &str {
    match *self {
      IldaError::FileTooSmall => "FileTooSmall",
      IldaError::InvalidData => "InvalidData",
      IldaError::InvalidHeader => "InvalidHeader",
      IldaError::IoError { .. } => "IoError",
    }
  }
}

impl Display for IldaError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let description = match *self {
      IldaError::FileTooSmall => "FileTooSmall",
      IldaError::InvalidData => "InvalidData",
      IldaError::InvalidHeader => "InvalidHeader",
      IldaError::IoError { .. } => "IoError",
    };
    write!(f, "{}", description)
  }
}

impl From<io::Error> for IldaError {
  fn from(error: io::Error) -> IldaError {
    IldaError::IoError { cause: error }
  }
}
