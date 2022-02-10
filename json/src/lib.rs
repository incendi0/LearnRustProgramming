#![allow(dead_code)]

use std::result;

mod error;
mod parser;
mod value;

pub use error::JsonError;

pub type Result<T> = result::Result<T, JsonError>;
