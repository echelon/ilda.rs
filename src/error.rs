// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io;

/// Ilda library errors.
#[derive(Debug)]
pub enum IldaError {
  InvalidFile { reason: String },
  InvalidFormat,
  IoError { cause: io::Error },
}

impl Error for IldaError {
  fn description(&self) -> &str {
    match *self {
      IldaError::InvalidFile { .. } => "InvalidFile",
      IldaError::InvalidFormat => "InvalidFormat",
      IldaError::IoError { .. } => "IoError",
    }
  }
}

impl Display for IldaError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let description = match *self {
      IldaError::InvalidFile { .. } => "InvalidFile",
      IldaError::InvalidFormat => "InvalidFormat",
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
