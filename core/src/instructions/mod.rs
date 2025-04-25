//! ## Dev Notes
//!
//! - For the `*IxData` struct, keep the encapsulated byte array private
//!   and only expose via `self.as_buf()` so that users cannot
//!   accidentally set the wrong discriminant or input invalid data

pub mod assign;
pub mod create_account;
pub mod transfer;

mod internal_utils;
