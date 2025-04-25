//! ## Dev Notes
//!
//! - For the `*IxData` struct, keep the encapsulate byte array private
//!   and only expose via `self.as_buf()` so that users cannot
//!   accidentally set the wrong discriminant or input invalid data

pub mod assign;

mod internal_utils;
