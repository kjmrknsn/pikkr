//! JSON parser which picks up values directly without performing tokenization
extern crate farmhash;
extern crate x86intrin;

mod avx;
mod bit;
mod error;
mod farmhash_collections;
mod index_builder;
mod parser;
mod pikkr;
mod query;
mod result;
mod utf8;

pub use error::{Error, ErrorKind};
pub use pikkr::Pikkr;
pub use result::Result;
