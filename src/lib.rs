
///! # shaun
///! `shaun` (standing for Shaun HAtes Ugly Notations) is a notation language based on
///! HJSON. This crate contains everything to manipulate SHAUN data.

#[macro_use]
extern crate serde_derive;
extern crate serde;

mod shaun_type;
mod parser;

// Shaun type exports
pub use shaun_type::Shaun;
pub use shaun_type::ShaunError;
pub use shaun_type::ShaunError::*;
pub use shaun_type::Shaun::*;

// parsing functions export
pub use parser::parse_str;
pub use parser::parse_string;
pub use parser::parse_file;
