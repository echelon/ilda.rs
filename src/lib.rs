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

#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]
#![deny(unused_qualifications)]

extern crate point;

pub mod animation;
pub mod data;
pub mod limit;
pub mod parser;
pub mod writer;

mod color;
mod error;

pub use error::IldaError;
pub use point::SimplePoint;
