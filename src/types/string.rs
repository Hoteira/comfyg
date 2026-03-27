#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

use super::ParserError;

pub fn parse_string(value: &str) -> Result<String, ParserError> {
    Ok(String::from(value))
}
