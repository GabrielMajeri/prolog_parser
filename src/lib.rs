extern crate num;
extern crate ordered_float;

// external parts (those used directly in rusty-wam)
#[macro_use] pub mod tabled_rc;
#[macro_use] pub mod ast;
#[macro_use] pub mod macros;
pub mod string_list;
pub mod parser;

// internal parts.
mod put_back_n;
mod lexer;