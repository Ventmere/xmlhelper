//! XML encode/decode utilities built on [xml-rs](https://github.com/netvl/xml-rs)
//!

#[macro_use]
extern crate error_chain;
extern crate xml;

pub mod decode;
pub mod encode;
