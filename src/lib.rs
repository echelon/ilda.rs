// Copyright (c) 2015 Brandon Thomas <bt@brand.io>

//! Ilda.rs is a library for parsing _International Laser Display Association_
//! (ILDA) display files. These files consist of RGB points, which may be
//! grouped into one or more frames. Points are sent to a laser projector
//! sequentially in order to render them as a static figure or animation.
//!
//! This library contains both a high-level and low-level interface for reading
//! ILDA files. The high-level interface is recommended, but the low level
//! API may be used in the future to serialize frames back into binary ILDA
//! files (TODO).

// TODO: Warn on missing documentation. #![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]

pub mod animation;
pub mod data;
pub mod limit;
pub mod parser;

mod color;
mod error;

pub use error::IldaError;
