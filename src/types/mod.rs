#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

pub enum Types {
    Color,
    Number,
    String,
    Bool,
}

#[derive(Debug, Clone)]
pub enum ReturnTypes {
    Bool(bool),
    Number(f64),
    String(String),
    Color([u8; 4]),
}

pub trait Parse {
    fn parse(&self, value: &str) -> Result<ReturnTypes, ParserError>;
}

impl Parse for Types {
    fn parse(&self, value: &str) -> Result<ReturnTypes, ParserError> {
        Ok(match self {
            Types::Color => ReturnTypes::Color(color::parse_color(value)?),
            Types::Number => ReturnTypes::Number(number::parse_number(value)?),
            Types::String => ReturnTypes::String(string::parse_string(value)?),
            Types::Bool => ReturnTypes::Bool(bool::parse_bool(value)?),
        })
    }
}

#[derive(Debug, Clone)]
pub enum ParserError {
    InvalidColor,
    InvalidNumber,
    InvalidBool,
}

pub mod bool;
pub mod color;
pub mod number;
pub mod string;
