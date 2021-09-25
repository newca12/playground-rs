#![feature(try_blocks)]

pub use tptp_parser::str_to_visit;

pub mod tptp_parser;
pub use grammar::*;
pub mod grammar;
